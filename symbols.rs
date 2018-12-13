
// List of Currency Status
enum status
{
    PRE_TRADING(String),
    TRADING(String),
    POST_TRADING(String),
    END_OF_DAY(String),
    HALT(String),
    AUCTION_MATCH(String),
    BREAK(String)
}

// List of Trade Duration
enum Duration
{
    GTC(String),
    IOC(String),
    FOK(String
}
 
// List of Orders
enum OrderSide
{
    Buy(String),
    Sell(String)
}

// Rate Limit Intervals
enum RateLimit
{
    Hours(String),
    Minutes(String),
    Seconds(String)
}

// Structure of Currency 
struct Currency 
{
    symbol: String,
    status: status, 
    price: f64
}

// Structure of an order
struct Order
{
    symbol: String,
    order_id: u64, 
    clientOrderid: String, 
    exchange: String, 
    price: f64,
    qty: u64,
    status: String,
    execQty: u64, 
    activeTime: Duration, 
    type: String, 
    action: OrderSide
}
  
// Structue of Trades
struct Trade
{
    symbol: String,
    order_id: u64, 
    clientOrderid: String, 
    exchange: String, 
    price: f64,
    qty: u64,
    status: String,
    execQty: u64, 
    activeTime: Duration, 
    type: String, 
    action: OrderSide
}



    
