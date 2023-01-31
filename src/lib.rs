use std::iter::Map;

struct TransactionId(String);
struct AccountId(String);
struct BucketId(String);
struct BookId(String);
struct UserId(String);
struct CurrencyId(String);
struct Money(String); // replace with proper decimal crate
struct Date(String);
struct TickerSymbol(String);
struct Quantity(String);

enum LedgerPermission {
  View,
}

// not explicitly differentiating assets and liabilities to avoid credit/debit terminology
enum AccountType {
  Cash,
  Credit,
  Brokerage,
}

struct Book {
  id: BookId,
  name: String,
  owner: UserId,
}

struct BookCollaborator {
  book_id: BookId,
  collaborator: UserId,
  permissions: Vec<LedgerPermission>,
}

// models assets & liabilities
// has only one currency
struct Account {
  account_id: AccountId,
  book_id: BookId,
  name: String,
  r#type: AccountType,
  currency: CurrencyId,
  // number of decimal places (e.g., 2 for AUD, 8 for BTC etc)
  // maybe should be defined by currency?
  // maybe should be used only for brokerage accounts to model fractional stock?
  precision: u8,
}

// models expenses & income/equity or other external value
struct Bucket {
  bucket_id: BucketId,
  book_id: BookId,
  name: String,
  currency: CurrencyId,
}

struct Transaction {
  transaction_id: TransactionId,
  date: Date,
  order: i32,
  notes: String,
  details: Vec<TransactionDetail>,
}

enum TransactionDetail {
  Transfer {
    from: AccountId,
    to: AccountId,
    from_amount: Money,
    to_amount: Money,
  },
  Income {
    from: BucketId,
    to: AccountId,
    amount: Money,
  },
  Expense {
    from: AccountId,
    to: BucketId,
    amount: Money,
  },
  // brokerage fees should be entered as a separate expense transaction
  Security {
    settlementAccount: AccountId,
    ticker: TickerSymbol,
    // can be fractional shares
    // define precision in settlementAccount? (for example stake allows fractional shares but nabtrade and commsec doesn't)
    quantity: Quantity,
    amount: Money,
  },
}

// assests - liabilities = income - expenses
// but since you can do currency conversions, this wont always be true
// how can we guarantee correctness with multiple currencies without having to recalculate?
struct Balances {
  floating_balances: Map<CurrencyId, Balance>,
  bucket_balances: Map<BucketId, Map<CurrencyId, Balance>>,
  account_balances: Map<AccountId, Balance>,
}

struct Balance {
  total_in: Money,  // similar to debits for asset accounts
  total_out: Money, // similar to credits for liability accounts
}
