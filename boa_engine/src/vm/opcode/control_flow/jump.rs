use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult, JsValue,
};

/// `Jump` implements the Opcode Operation for `Opcode::Jump`
///
/// Operation:
///  - Unconditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Jump;

impl Operation for Jump {
    const NAME: &'static str = "Jump";
    const INSTRUCTION: &'static str = "INST - Jump";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        context.vm.frame_mut().pc = address;
        Ok(CompletionType::Normal)
    }
}

// `JumpIfTrue` implements the Opcode Operation for `Opcode::JumpIfTrue`
///
/// Operation:
///  - Conditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct JumpIfTrue;

impl Operation for JumpIfTrue {
    const NAME: &'static str = "JumpIfTrue";
    const INSTRUCTION: &'static str = "INST - JumpIfTrue";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        if context.vm.pop().to_boolean() {
            context.vm.frame_mut().pc = address;
        }
        Ok(CompletionType::Normal)
    }
}

/// `JumpIfFalse` implements the Opcode Operation for `Opcode::JumpIfFalse`
///
/// Operation:
///  - Conditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct JumpIfFalse;

impl Operation for JumpIfFalse {
    const NAME: &'static str = "JumpIfFalse";
    const INSTRUCTION: &'static str = "INST - JumpIfFalse";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        if !context.vm.pop().to_boolean() {
            context.vm.frame_mut().pc = address;
        }
        Ok(CompletionType::Normal)
    }
}

/// `JumpIfNotUndefined` implements the Opcode Operation for `Opcode::JumpIfNotUndefined`
///
/// Operation:
///  - Conditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct JumpIfNotUndefined;

impl Operation for JumpIfNotUndefined {
    const NAME: &'static str = "JumpIfNotUndefined";
    const INSTRUCTION: &'static str = "INST - JumpIfNotUndefined";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        let value = context.vm.pop();
        if !value.is_undefined() {
            context.vm.frame_mut().pc = address;
            context.vm.push(value);
        }
        Ok(CompletionType::Normal)
    }
}

/// `JumpIfNullOrUndefined` implements the Opcode Operation for `Opcode::JumpIfNullOrUndefined`
///
/// Operation:
///  - Conditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct JumpIfNullOrUndefined;

impl Operation for JumpIfNullOrUndefined {
    const NAME: &'static str = "JumpIfNullOrUndefined";
    const INSTRUCTION: &'static str = "INST - JumpIfNullOrUndefined";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        let value = context.vm.pop();
        if value.is_null_or_undefined() {
            context.vm.frame_mut().pc = address;
        } else {
            context.vm.push(value);
        }
        Ok(CompletionType::Normal)
    }
}

/// `JumpTable` implements the Opcode Operation for `Opcode::JumpTable`
///
/// Operation:
///  - Conditional jump to address.
#[derive(Debug, Clone, Copy)]
pub(crate) struct JumpTable;

impl Operation for JumpTable {
    const NAME: &'static str = "JumpTable";
    const INSTRUCTION: &'static str = "INST - JumpTable";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let default = context.vm.read::<u32>();
        let count = context.vm.read::<u32>();

        let value = context.vm.pop();
        if let JsValue::Integer(value) = &value {
            let value = *value as u32;
            let mut target = None;
            for i in 0..count {
                let address = context.vm.read::<u32>();
                if i + 1 == value {
                    target = Some(address);
                }
            }

            context.vm.frame_mut().pc = target.unwrap_or(default);

            return Ok(CompletionType::Normal);
        }

        unreachable!("expected positive integer, got {value:?}")
    }
}
