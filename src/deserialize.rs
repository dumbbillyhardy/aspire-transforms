mod protos;
use protos::aspire::{YnabTransaction, YnabTransaction_Status, MintTransaction, MintTransaction_TransactionType, AspireTransaction, AspireTransaction_Status};

fn deserialize_mint(csv: string) -> MintTransaction {
    let mut mint = MintTransaction::new();

    let data = csv.split(",");
    //mint.set_date(data[0]);
    mint.set_description(data[1]);
    mint.set_original_description(data[2]);
    //TODO atod
    //mint.set_amount(data[3]);
    if(data[4] == "debit") {
        mint.set_transaction_type(MintTransaction_TransactionType::DEBIT);
    } else if(data[4] == "credit") {
        mint.set_transaction_type(MintTransaction_TransactionType::CREDIT);
    } else {
        mint.set_transaction_type(MintTransaction_TransactionType::UNKNOWN);
    }

    mint.set_category(data[5]);
    mint.set_account_name(data[6]);
    mint.set_labels(data[7]);
    mint.set_notes(data[8]);
    
    return mint;
}
