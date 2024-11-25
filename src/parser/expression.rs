use std::{collections::HashMap, sync::Arc};

use winnow::{
    combinator::{self, alt},
    PResult, Parser,
};

use crate::types::expression::{
    BinaryOp, Combine, Expression, ExpressionWithWidth, Extract, ExtractInner, ModuleUse, UnaryOp,
};

use super::{
    argument::parse_arguments_inner,
    identifier::parse_identifier,
    number::parse_number,
    trivial_tokens::{
        parse_amperstand, parse_bang, parse_bang_amperstand, parse_bang_caret, parse_bang_pipe,
        parse_caret, parse_close_paren, parse_close_square_bracket, parse_colon, parse_comma,
        parse_dot, parse_double_dot, parse_open_paren, parse_open_square_bracket, parse_pipe,
    },
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_binary_expression.parse_next(input)
}

pub fn parse_postfix_operator(input: &mut Stream) -> PResult<String> {
    parse_whitespace(input)?;

    combinator::alt((parse_dot, parse_open_paren))
        .map(|s| s.to_string())
        .parse_next(input)
}

pub fn parse_term(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    let expression = alt((
        parse_variable_expression,
        parse_integer_expression,
        parse_combine_expression,
        parse_paren_expression,
    ))
    .parse_next(input)?;

    let postfix = parse_postfix_operator(input);

    if let Ok(postfix) = postfix {
        return match postfix.as_str() {
            "." => {
                let extract = parse_extract(input)?;

                Ok(Expression::Extract(Extract {
                    expression: Arc::new(ExpressionWithWidth::new(expression, &input.state)),
                    extract,
                }))
            }
            "(" => {
                let arguments = combinator::terminated(parse_arguments_inner, parse_close_paren)
                    .parse_next(input)?;

                if let Expression::Variable(expression) = expression {
                    Ok(Expression::ModuleUse(ModuleUse {
                        name: expression,
                        arguments,
                    }))
                } else {
                    Err(winnow::error::ErrMode::Backtrack(
                        winnow::error::ContextError::new(),
                    ))
                }
            }
            _ => Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            )),
        };
    } else {
        Ok(expression)
    }
}

pub fn parse_factor(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    combinator::alt((parse_term, parse_unary_expression)).parse_next(input)
}

fn parse_paren_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    combinator::delimited(parse_open_paren, parse_expression, parse_close_paren).parse_next(input)
}

fn parse_integer_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_number.map(Expression::Integer).parse_next(input)
}

fn parse_variable_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_identifier
        .map(|s| Expression::Variable(s.to_string()))
        .parse_next(input)
}

fn parse_binary_operator(input: &mut Stream) -> PResult<String> {
    parse_whitespace(input)?;

    alt((
        parse_amperstand,
        parse_pipe,
        parse_caret,
        parse_bang_amperstand,
        parse_bang_pipe,
        parse_bang_caret,
    ))
    .map(|s| s.to_string())
    .parse_next(input)
}

fn parse_binary_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    let mut lhs = parse_factor(input)?;

    let half_binary_expressions: Vec<_> = combinator::repeat_till(
        0..,
        parse_half_binary_operation,
        combinator::not(parse_binary_operator),
    )
    .map(|v| v.0)
    .parse_next(input)?;

    for half_binary_expression in half_binary_expressions {
        let rhs = half_binary_expression.rhs;
        let op = match half_binary_expression.op.as_str() {
            "&" => BinaryOp::And(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            "|" => BinaryOp::Or(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            "^" => BinaryOp::XOr(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            "!&" => BinaryOp::NAnd(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            "!|" => BinaryOp::NOr(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            "!^" => BinaryOp::XNOr(
                Arc::new(ExpressionWithWidth::new(lhs, &input.state)),
                Arc::new(ExpressionWithWidth::new(rhs, &input.state)),
            ),
            _ => unreachable!(),
        };

        lhs = Expression::BinaryOp(op);
    }

    Ok(lhs)
}

struct HalfBinaryOp {
    rhs: Expression,
    op: String,
}

fn parse_half_binary_operation(input: &mut Stream) -> PResult<HalfBinaryOp> {
    parse_whitespace(input)?;

    let op = parse_binary_operator(input)?;
    let rhs = parse_factor(input)?;
    Ok(HalfBinaryOp { rhs, op })
}

fn parse_unary_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    let (op, expr) = (alt((parse_bang,)), parse_expression).parse_next(input)?;

    match op {
        "!" => Ok(Expression::UnaryOp(UnaryOp::Not(Arc::new(
            ExpressionWithWidth::new(expr, &input.state),
        )))),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

fn parse_range(input: &mut Stream) -> PResult<Range> {
    parse_whitespace(input)?;

    let (start, _, end) = (parse_number, parse_double_dot, parse_number).parse_next(input)?;

    Ok(Range { start, end })
}

fn parse_bit_extract(input: &mut Stream) -> PResult<ExtractInner> {
    parse_whitespace(input)?;

    parse_number.map(ExtractInner::Bit).parse_next(input)
}

fn parse_name_extract(input: &mut Stream) -> PResult<ExtractInner> {
    parse_whitespace(input)?;

    parse_identifier
        .map(|s| ExtractInner::Name(s.to_string()))
        .parse_next(input)
}

fn parse_range_extract(input: &mut Stream) -> PResult<ExtractInner> {
    parse_whitespace(input)?;

    (parse_number, parse_double_dot, parse_number)
        .map(|(start, _, end)| ExtractInner::Range(start, end))
        .parse_next(input)
}

fn parse_extract(input: &mut Stream) -> PResult<ExtractInner> {
    parse_whitespace(input)?;

    alt((parse_range_extract, parse_bit_extract, parse_name_extract)).parse_next(input)
}

#[derive(Debug)]
enum CombineKey {
    Number(u32),
    MultiNumber(Vec<u32>),
    NumberRange(Range),
    Identifier(String),
    MultiIdentifier(Vec<String>),
}

fn parse_multi_number(input: &mut Stream) -> PResult<Vec<u32>> {
    parse_whitespace(input)?;

    let numbers = combinator::separated(1.., parse_number, parse_comma).parse_next(input)?;

    Ok(numbers)
}
fn parse_multi_identifier(input: &mut Stream) -> PResult<Vec<String>> {
    parse_whitespace(input)?;

    let identifiers =
        combinator::separated(1.., parse_identifier.map(|s| s.to_string()), parse_comma)
            .parse_next(input)?;

    Ok(identifiers)
}

fn parse_combine_key(input: &mut Stream) -> PResult<CombineKey> {
    parse_whitespace(input)?;

    alt((
        parse_multi_identifier.map(CombineKey::MultiIdentifier),
        parse_multi_number.map(CombineKey::MultiNumber),
        parse_range.map(CombineKey::NumberRange),
    ))
    .parse_next(input)
}

#[derive(Debug)]
struct CombineKV {
    key: CombineKey,
    value: Expression,
}

fn parse_combine_kv(input: &mut Stream) -> PResult<CombineKV> {
    parse_whitespace(input)?;

    combinator::seq!(CombineKV {
        key: parse_combine_key,
        _: parse_colon,
        value: parse_expression,
    })
    .parse_next(input)
}

fn parse_combine_expression(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_open_square_bracket(input)?;

    let kvs: Vec<_> =
        combinator::separated(0.., parse_combine_kv, parse_comma).parse_next(input)?;

    combinator::opt(parse_comma).parse_next(input)?; // optional trailing comma

    parse_close_square_bracket(input)?;

    if kvs.is_empty() {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    let first_kv = &kvs[0];
    match first_kv.key {
        CombineKey::Number(_) | CombineKey::MultiNumber(_) | CombineKey::NumberRange(_) => {
            let mut map = HashMap::new();

            for kv in kvs {
                match kv.key {
                    CombineKey::Number(n) => {
                        map.insert(n, kv.value.clone());
                    }
                    CombineKey::MultiNumber(ns) => {
                        for n in ns {
                            map.insert(n, kv.value.clone());
                        }
                    }
                    CombineKey::NumberRange(range) => {
                        for (i, n) in (range.start..=range.end).enumerate() {
                            let expression = kv.value.clone();
                            map.insert(
                                n,
                                Expression::Extract(Extract {
                                    expression: Arc::new(ExpressionWithWidth::new(
                                        expression,
                                        &input.state,
                                    )),
                                    extract: ExtractInner::Bit(i.try_into().unwrap()),
                                }),
                            );
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let largest = *map.keys().max().unwrap();

            let mut values = Vec::new();
            for i in 0..=largest {
                if map.contains_key(&i) {
                    values.push(map.get(&i).unwrap().clone());
                } else {
                    values.push(Expression::Integer(0));
                }
            }

            Ok(Expression::Combine(Combine::Bits(values)))
        }
        CombineKey::Identifier(_) | CombineKey::MultiIdentifier(_) => {
            let mut map = HashMap::new();

            for kv in kvs {
                match kv.key {
                    CombineKey::Identifier(s) => {
                        map.insert(s, kv.value.clone());
                    }
                    CombineKey::MultiIdentifier(ss) => {
                        for s in ss {
                            map.insert(s, kv.value.clone());
                        }
                    }
                    _ => unreachable!(),
                }
            }

            Ok(Expression::Combine(Combine::Obj(map)))
        }
    }
}
