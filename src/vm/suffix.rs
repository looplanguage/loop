use crate::object::integer::Integer;
use crate::object::Object;
use crate::vm::VM;

pub fn run_suffix_expression(vm: &mut VM, operator: &str) {
    let right = vm.pop();
    let left = vm.pop();

    match operator {
        "+" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Object::Integer(Integer {
                        value: left_obj.value + right_obj.value,
                    }));
                };
            };
        }
        "*" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Object::Integer(Integer {
                        value: left_obj.value * right_obj.value,
                    }));
                };
            };
        }
        "-" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Object::Integer(Integer {
                        value: left_obj.value - right_obj.value,
                    }));
                };
            };
        }
        "%" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Object::Integer(Integer {
                        value: left_obj.value % right_obj.value,
                    }));
                };
            };
        }
        "/" => {
            if let Object::Integer(left_obj) = left {
                if let Object::Integer(right_obj) = right {
                    vm.push(Object::Integer(Integer {
                        value: left_obj.value / right_obj.value,
                    }));
                };
            };
        }
        _ => {
            panic!(format!("unknown operator {}", operator))
        }
    }
}
