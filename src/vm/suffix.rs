use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::Object;
use crate::vm::VM;
use std::rc::Rc;

pub fn run_suffix_expression(vm: &mut VM, operator: &str) -> Option<String> {
    let right = &*vm.pop();
    let left = &*vm.pop();

    // TODO: Clean this up a little
    match operator {
        "+" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Integer(Integer {
                        value: left_obj.value + right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        "*" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Integer(Integer {
                        value: left_obj.value * right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        "-" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Integer(Integer {
                        value: left_obj.value - right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        "%" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Integer(Integer {
                        value: left_obj.value % right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        "/" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Integer(Integer {
                        value: left_obj.value / right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        "==" => {
            vm.push(Rc::new(Object::Boolean(Boolean {
                value: left == right,
            })));
        }
        "!=" => {
            vm.push(Rc::new(Object::Boolean(Boolean {
                value: left != right,
            })));
        }
        ">" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Rc::new(Object::Boolean(Boolean {
                        value: left_obj.value > right_obj.value,
                    })));
                } else {
                    return Some(format!(
                        "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                        right
                    ));
                }
            } else {
                return Some(format!(
                    "unexpected type. got=\"{:?}\". expected=\"Integer\"",
                    left
                ));
            }
        }
        _ => return Some(format!("unknown operator {}", operator)),
    }

    None
}
