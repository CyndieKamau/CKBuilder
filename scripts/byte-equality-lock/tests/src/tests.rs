use ckb_testtool::{
    ckb_script::{ScriptError, TransactionScriptError},
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder, TransactionView},
        packed::*,
        prelude::*,
    },
};
use ckb_testtool::context::Context;

const SECRET: &[u8] = b"open sesame";
const WRONG_SECRET_EXIT_CODE: i8 = 5;

fn build_unlock_tx(witness_args: WitnessArgs) -> (Context, TransactionView) {
    let mut context = Context::default();
    let out_point = context.deploy_cell_by_name("byte-equality-lock");

    let lock_script = context
        .build_script(&out_point, Bytes::from(SECRET.to_vec()))
        .expect("script");

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64)
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64)
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64)
            .lock(lock_script)
            .build(),
    ];
    let outputs_data = vec![Bytes::new(); 2];

    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .witness(witness_args.as_bytes().pack())
        .build();
    let tx = context.complete_tx(tx);

    (context, tx)
}

fn build_unlock_tx_with_witness_lock(witness_lock: Bytes) -> (Context, TransactionView) {
    let witness_args = WitnessArgs::new_builder()
        .lock(Some(witness_lock).pack())
        .build();

    build_unlock_tx(witness_args)
}

fn assert_wrong_secret_error(err: ckb_testtool::ckb_error::Error) {
    let script_error = err
        .root_cause()
        .downcast_ref::<TransactionScriptError>()
        .expect("transaction script error");

    assert_eq!(
        script_error.originating_script().to_string(),
        "Inputs[0].Lock",
    );

    match script_error.script_error() {
        ScriptError::ValidationFailure(_, code) => assert_eq!(*code, WRONG_SECRET_EXIT_CODE),
        err => panic!("expected validation failure, got {err:?}"),
    }
}

#[test]
fn byte_equality_lock_accepts_matching_witness_lock() {
    let (context, tx) = build_unlock_tx_with_witness_lock(Bytes::from(SECRET.to_vec()));

    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

#[test]
fn byte_equality_lock_rejects_wrong_witness_lock() {
    let (context, tx) = build_unlock_tx_with_witness_lock(Bytes::from(b"wrong secret".to_vec()));

    let err = context
        .verify_tx(&tx, 10_000_000)
        .expect_err("wrong witness lock should fail");

    assert_wrong_secret_error(err);
}

#[test]
fn byte_equality_lock_rejects_missing_witness_lock() {
    let (context, tx) = build_unlock_tx(WitnessArgs::new_builder().build());

    let err = context
        .verify_tx(&tx, 10_000_000)
        .expect_err("missing witness lock should fail");

    assert_wrong_secret_error(err);
}
