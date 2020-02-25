mod protos;
use protos::aspire::{MintTransaction, MintTransaction_TransactionType, AspireTransaction, AspireTransaction_Status};

fn transform(mint: MintTransaction) -> AspireTransaction {
    let mut aspire = AspireTransaction::new();

    //aspire.set_date(mint.get_date());
    if(mint.get_transaction_type() == MintTransaction_TransactionType::DEBIT) {
      aspire.set_outflow(mint.get_amount());
    }
    else if(mint.get_transaction_type() == MintTransaction_TransactionType::CREDIT) {
      aspire.set_inflow(mint.get_amount());
    }
    aspire.set_category(mint.get_category());
    aspire.set_account(mint.get_account_name());
    aspire.set_memo("");
    //TODO: figure out a way to infer this?
    aspire.set_status(AspireTransaction_Status::UNKNOWN);
    aspire.set_approved(false);

    return aspire;
}
