use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::backend::transfers::Transfer as TransferDto;
use crate::models::commons::Operation::DELEGATE;
use crate::models::commons::ParamValue::SingleValue;
use crate::models::commons::{DataDecoded, Operation, Parameter};
use crate::models::service::transactions::details::{
    DetailedExecutionInfo, ModuleExecutionDetails, MultisigConfirmation, MultisigExecutionDetails,
    TransactionData, TransactionDetails,
};
use crate::models::service::transactions::{
    Custom, Erc721Transfer, TransactionInfo, TransactionStatus, Transfer, TransferDirection,
    TransferInfo,
};
use crate::providers::info::*;

#[test]
fn multisig_custom_transaction_to_transaction_details() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_CUSTOM).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let timestamp_confirmation0: i64 = 1592837914055;
    let timestamp_confirmation1: i64 = 1592838142231;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .returning(move |_| anyhow::bail!("Token Address 0x0"));

    let expected = TransactionDetails {
        executed_at: multisig_tx.execution_date.map(|it| it.timestamp_millis()),
        tx_status: TransactionStatus::Success,
        tx_hash: Some("0x0ebb2c317f55c96469e0ed2014f5833dc02a70b42f0ac52f4630938900caa698".to_string()),
        tx_info: TransactionInfo::Custom(Custom {
            to: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
            data_size: "68".to_string(),
            value: "0".to_string(),
            method_name: Some("approve".to_string()),
        }),
        tx_data: Some(TransactionData {
            hex_data: Some(String::from("0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000")),
            data_decoded: Some(DataDecoded {
                method: "approve".to_string(),
                parameters: Some(vec![
                    Parameter {
                        name: "spender".to_string(),
                        param_type: "address".to_string(),
                        value: SingleValue(String::from("0xae9844F89D98c150F5e61bfC676D68b492155990")),
                        value_decoded: None,
                    },
                    Parameter {
                        name: "value".to_string(),
                        param_type: "uint256".to_string(),
                        value: SingleValue(String::from("500000000000000")),
                        value_decoded: None,
                    }
                ]),
            }),
            to: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
            value: Some(String::from("0")),
            operation: Operation::CALL,
        }),
        detailed_execution_info: Some(DetailedExecutionInfo::Multisig(
            MultisigExecutionDetails {
                submitted_at: multisig_tx.submission_date.timestamp_millis(),
                nonce: 84,
                safe_tx_gas: 43485,
                base_gas: 0,
                gas_price: "0".to_string(),
                gas_token: "0x0000000000000000000000000000000000000000".to_string(),
                refund_receiver: "0x0000000000000000000000000000000000000000".to_string(),
                safe_tx_hash: "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621".to_string(),
                executor: Some("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string()),
                signers: vec![
                    String::from("0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"),
                    String::from("0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72"),
                    String::from("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"),
                    String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"),
                    String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0")],
                confirmations_required: 2,
                confirmations: vec![
                    MultisigConfirmation {
                        signer: String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
                        signature: Some(String::from("0x83b1506c409918f21031e93ed2f62310a5e0c05b1be89242a6a266a7de4af7bc6094e206b33387b8d4465af6087a4d2158815e613aeb186d88d9a1973e00bbe81b")),
                        submitted_at: timestamp_confirmation0,
                    },
                    MultisigConfirmation {
                        signer: String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"),
                        signature: Some(String::from("0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001")),
                        submitted_at: timestamp_confirmation1,
                    },
                ],
                gas_token_info: None
            })),
        safe_app_info: None
    };

    let actual = MultisigTransaction::to_transaction_details(&multisig_tx, &mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn module_transaction_to_transaction_details() {
    let module_transaction =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX).unwrap();

    let expected = TransactionDetails {
        executed_at: Some(module_transaction.execution_date.timestamp_millis()),
        tx_status: TransactionStatus::Success,
        tx_hash: Some("0x705167e310ef0acb80a5f73eb4f8e66cfb32a896ac9380f3eb43e68ef8603a9f".to_string()),
        tx_info: TransactionInfo::Custom(Custom {
            to: "0xaAEb2035FF394fdB2C879190f95e7676f1A9444B".to_string(),
            data_size: "132".to_string(),
            value: "0".to_string(),
            method_name: None,
        }),
        tx_data: Some(TransactionData {
            hex_data: Some(String::from("0x59f96ae500000000000000000000000000df91984582e6e96288307e9c2f20b38c8fece9000000000000000000000000c778417e063141139fce010982780140aa0cd5ab0000000000000000000000000000000000000000000000000000000000000475000000000000000000000000000000000000000000000003d962c8be3053def2")),
            data_decoded: None,
            to: "0xaAEb2035FF394fdB2C879190f95e7676f1A9444B".to_string(),
            value: Some(String::from("0")),
            operation: Operation::CALL,
        }),
        detailed_execution_info: Some(DetailedExecutionInfo::Module(
            ModuleExecutionDetails {
                address: "0xfa559f0932b7B60d90B4af0b8813d4088465096b".to_string()
            })),
        safe_app_info: None
    };

    let actual = ModuleTransaction::to_transaction_details(&module_transaction);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn ethereum_tx_transfer_to_transaction_details() {
    let transfer =
        serde_json::from_str::<TransferDto>(crate::json::ERC_20_TRANSFER_WITH_ERC721_TOKEN_INFO)
            .unwrap();

    let expected = TransactionDetails {
        executed_at: Some(transfer.get_execution_time().unwrap()),
        tx_status: TransactionStatus::Success,
        tx_hash: Some(
            "0x317db9d079e46fef2f758e37bd20efb14d5c83e2510307079207bc6f04cdee48".to_string(),
        ),
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0xd31e655bC4Eb5BCFe25A47d636B25bb4aa4041B2".to_string(),
            recipient: "0xBc79855178842FDBA0c353494895DEEf509E26bB".to_string(),
            direction: TransferDirection::Incoming,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: "0xa9517B2E61a57350D6555665292dBC632C76adFe".to_string(),
                token_id: "856420144564".to_string(),
                token_name: Some("a!NEVER VISIT www.168pools.com to check DeFi ROi !".to_string()),
                token_symbol: Some("a!NEVER VISIT www.168pools.com to check DeFi ROi !".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xa9517B2E61a57350D6555665292dBC632C76adFe.png".to_string()),
            }),
        }),
        tx_data: None,
        detailed_execution_info: None,
        safe_app_info: None
    };

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let actual = TransferDto::to_transaction_details(
        &transfer,
        &mut mock_info_provider,
        "0xBc79855178842FDBA0c353494895DEEf509E26bB",
    );

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn multisig_transaction_with_origin() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_WITH_ORIGIN).unwrap();
    let mut safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    // Lower nonce so that transaction is pending again
    safe_info.nonce = 140;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));
    mock_info_provider
        .expect_safe_app_info()
        .times(1)
        .return_once(move |_| {
            Ok(SafeAppInfo {
                name: "WalletConnect".to_string(),
                url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
                logo_url: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
            })
        });
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionDetails {
        executed_at: Some(multisig_tx.execution_date.unwrap().timestamp_millis()),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Custom(Custom {
            to: "0x8D29bE29923b68abfDD21e541b9374737B49cdAD".to_string(),
            data_size: "3108".to_string(),
            value: "0".to_string(),
            method_name: Some("multiSend".to_string()),
        }),
        tx_data: Some(TransactionData {
            hex_data: Some("0x8d80ff0a00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000bd900111111125434b319222cdbf8c261674adb56f3ae000000000000000000000000000000000000000000000ed2b525841adfc000000000000000000000000000000000000000000000000000000000000000000b8490411a32000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000ecee9b38efb1a680000000000000000000000000000000000000000000000000ed2b525841adfc000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000440000000000000000000000000000000000000000000000000000000000000076000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000324b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000001400000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000001e45636885000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000ecee9b38efb1a68000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed2b525841adfc0000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004d0e30db0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000002647f8fe7a000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000044000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe500000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000a405971224000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000002f9ae7c8305c3600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004470bdb947000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000184b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000100000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000bc79855178842fdba0c353494895deef509e26bb0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()),
            data_decoded: Some(DataDecoded {
                method: "multiSend".to_string(),
                parameters: Some(vec![Parameter {
                    name: "transactions".to_string(),
                    param_type: "bytes".to_string(),
                    value: SingleValue("0x00111111125434b319222cdbf8c261674adb56f3ae000000000000000000000000000000000000000000000ed2b525841adfc000000000000000000000000000000000000000000000000000000000000000000b8490411a32000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000ecee9b38efb1a680000000000000000000000000000000000000000000000000ed2b525841adfc000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000440000000000000000000000000000000000000000000000000000000000000076000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000324b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000001400000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000001e45636885000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000ecee9b38efb1a68000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed2b525841adfc0000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004d0e30db0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000002647f8fe7a000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000044000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe500000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000a405971224000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000002f9ae7c8305c3600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004470bdb947000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000184b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000100000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000bc79855178842fdba0c353494895deef509e26bb00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()),
                    value_decoded: None }]) }),
            to: "0x8D29bE29923b68abfDD21e541b9374737B49cdAD".to_string(),
            value: Some("0".to_string()),
            operation: DELEGATE }),
        detailed_execution_info: Some(DetailedExecutionInfo::Multisig(
            MultisigExecutionDetails {
                submitted_at: 1607346684674,
                nonce: 160,
                safe_tx_gas: 124625,
                base_gas: 0,
                gas_price: "0".to_string(),
                gas_token: "0x0000000000000000000000000000000000000000".to_string(),
                refund_receiver: "0x0000000000000000000000000000000000000000".to_string(),
                safe_tx_hash: "0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80".to_string(),
                executor: Some("0xe965484BA4250c446779D4703f1598DC2EA00d12".to_string()),
                signers: vec!["0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
                              "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_string(),
                              "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
                              "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
                              "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string()],
                confirmations_required: 2,
                confirmations: vec![
                    MultisigConfirmation {
                        signer: "0xc9d486048A9B82172F6f2A2ce6D9024c9D1097dC".to_string(),
                        signature: Some("0xbd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c".to_string()),
                        submitted_at: 1607346684686,
                    },
                     MultisigConfirmation {
                         signer: "0xe965484BA4250c446779D4703f1598DC2EA00d12".to_string(),
                         signature: Some("0x000000000000000000000000e965484ba4250c446779d4703f1598dc2ea00d12000000000000000000000000000000000000000000000000000000000000000001".to_string()),
                         submitted_at: 1607346715000,
                     }
                ],
                gas_token_info: None,
            })),
        safe_app_info: Some(SafeAppInfo {
            name: "WalletConnect".to_string(),
            url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
            logo_url: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
        }),
        tx_hash: Some("0x4d84602bf94d099159baa41993edca288abb5f9795dd51cc14bd66195b9fdc77".to_string()),
    };

    let actual = MultisigTransaction::to_transaction_details(&multisig_tx, &mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}
