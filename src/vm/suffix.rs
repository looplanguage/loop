use crate::lib::object::boolean::Boolean;
use crate::lib::object::float::Float;
use crate::lib::object::Object;
use crate::vm::VM;
use std::rc::Rc;

pub fn run_suffix_expression(vm: &mut VM, operator: &str) -> Option<String> {
    let right = &*vm.pop();
    let left = &*vm.pop();

    match operator {
        "==" => {
            vm.push(Rc::from(Object::Boolean(Boolean {
                value: left == right,
            })));
            return None;
        }
        "!=" => {
            vm.push(Rc::from(Object::Boolean(Boolean {
                value: left != right,
            })));
            return None;
        }
        _ => {}
    }

    let right_obj = match right {
        Object::Float(float) => *float,
        Object::Integer(integer) => integer.to_float(),
        _ => return Some(format!("unexpected type. got=\"{:?}\"", right)),
    };

    let left_obj = match left {
        Object::Float(float) => *float,
        Object::Integer(integer) => integer.to_float(),
        _ => return Some(format!("unexpected type. got=\"{:?}\"", right)),
    };

    // + - / * %

    // TODO: Clean this up a little
    let obj = match operator {
        ">" => Object::Boolean(Boolean {
            value: left_obj.value > right_obj.value,
        }),
        "+" => Object::Float(Float {
            value: left_obj.value + right_obj.value,
        }),
        "-" => Object::Float(Float {
            value: left_obj.value - right_obj.value,
        }),
        "/" => Object::Float(Float {
            value: left_obj.value / right_obj.value,
        }),
        "*" => Object::Float(Float {
            value: left_obj.value * right_obj.value,
        }),
        "%" => Object::Float(Float {
            value: left_obj.value % right_obj.value,
        }),
        _ => return Some(format!("unknown operator {}", operator)),
    };

    let push_object: Object = match obj {
        Object::Integer(_) => obj,
        Object::Boolean(_) => obj,
        Object::Float(float) => {
            if float.value == (float.value as u32) as f64 {
                Object::Integer(float.to_integer())
            } else {
                obj
            }
        }
        _ => {
            return Some(format!(
                "unknown return object for suffix expression. got=\"{:?}\"",
                obj
            ))
        }
    };

    vm.push(Rc::from(push_object));

    None
}
