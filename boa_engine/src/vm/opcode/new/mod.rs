use crate::{
    error::JsNativeError,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `New` implements the Opcode Operation for `Opcode::New`
///
/// Operation:
///  - Call construct on a function.
#[derive(Debug, Clone, Copy)]
pub(crate) struct New;

impl New {
    fn operation(context: &mut Context<'_>, argument_count: usize) -> JsResult<CompletionType> {
        let mut arguments = Vec::with_capacity(argument_count);
        for _ in 0..argument_count {
            arguments.push(context.vm.pop());
        }
        arguments.reverse();
        let func = context.vm.pop();

        let result = func
            .as_constructor()
            .ok_or_else(|| {
                JsNativeError::typ()
                    .with_message("not a constructor")
                    .into()
            })
            .and_then(|cons| cons.__construct__(&arguments, cons, context))?;

        context.vm.push(result);
        Ok(CompletionType::Normal)
    }
}

impl Operation for New {
    const NAME: &'static str = "New";
    const INSTRUCTION: &'static str = "INST - New";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let argument_count = context.vm.read::<u8>() as usize;
        Self::operation(context, argument_count)
    }

    fn execute_with_u16_operands(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let argument_count = context.vm.read::<u16>() as usize;
        Self::operation(context, argument_count)
    }

    fn execute_with_u32_operands(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let argument_count = context.vm.read::<u32>() as usize;
        Self::operation(context, argument_count)
    }
}

/// `NewSpread` implements the Opcode Operation for `Opcode::NewSpread`
///
/// Operation:
///  - Call construct on a function where the arguments contain spreads.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NewSpread;

impl Operation for NewSpread {
    const NAME: &'static str = "NewSpread";
    const INSTRUCTION: &'static str = "INST - NewSpread";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        // Get the arguments that are stored as an array object on the stack.
        let arguments_array = context.vm.pop();
        let arguments_array_object = arguments_array
            .as_object()
            .expect("arguments array in call spread function must be an object");
        let arguments = arguments_array_object
            .borrow()
            .properties()
            .dense_indexed_properties()
            .expect("arguments array in call spread function must be dense")
            .clone();

        let func = context.vm.pop();

        let result = func
            .as_constructor()
            .ok_or_else(|| {
                JsNativeError::typ()
                    .with_message("not a constructor")
                    .into()
            })
            .and_then(|cons| cons.__construct__(&arguments, cons, context))?;

        context.vm.push(result);
        Ok(CompletionType::Normal)
    }
}
