mod protos;
use protos::aspire::{YnabTransaction, YnabTransaction_Status, MintTransaction, MintTransaction_TransactionType, AspireTransaction, AspireTransaction_Status};

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

fn transform(ynab: YnabTransaction) -> AspireTransaction {
    let mut aspire = AspireTransaction::new();

    //aspire.set_date(ynab.get_date());
    aspire.set_outflow(ynab.get_outflow());
    aspire.set_inflow(ynab.get_inflow());
    aspire.set_category(ynab.get_category_group() + ": " + ynab.get_category());
    aspire.set_account(ynab.get_account());
    aspire.set_memo(ynab.get_memo());

    match ynab.get_status() {
        YnabTransaction_Status::UNCLEARED => {
            aspire.set_status(AspireTransaction_Status::UNCLEARED);
            aspire.set_approved(false);
        }
        YnabTransaction_Status::CLEARED => {
            aspire.set_status(AspireTransaction_Status::CLEARED);
            aspire.set_approved(true);
        }
        YnabTransaction_Status::RECONCILED => {
            aspire.set_status(AspireTransaction_Status::RECONCILED);
            aspire.set_approved(true);
        }
        YnabTransaction_Status::UNKNOWN | _ => {
            aspire.set_status(AspireTransaction_Status::UNKNOWN);
            aspire.set_approved(false);
        }
    }


    return aspire;
}
