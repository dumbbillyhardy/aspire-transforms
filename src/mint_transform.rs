mod protos;
use protos::aspire::{MintTransaction, AspireTransaction};

fn transform(mint: MintTransaction) -> AspireTransaction {
    let mut aspire = AspireTransaction::new();

    //aspire.set_date(mint.get_date());
    if(mint.get_transaction_type() == 1) {
      aspire.set_inflow(mint.get_amount());
    }
    else {
      aspire.set_outflow(mint.get_amount());
    }
    aspire.set_category(mint.get_category());
    aspire.set_account(mint.get_account_name());
    aspire.set_memo("");
    if (mint.get_
    aspire.set_status();


    return aspire;
}
