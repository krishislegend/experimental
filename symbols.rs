extern crate chrono;
use chrono::prelude::*;
use time::Duration;

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
    
// Order Fills 
enum fills
{
    Price(String),
    Qty(u64),
    CommFee(u64),
    CommCurr(String)
}

// Type of orders 
enum Type
{
    Bids(String),
    Asks(String)
}

// Structure of Currency 
struct Currency 
{
    symbol: String,
    depstatus: status, 
    precision:u8,
    price: f64,
    withdrawalfee: u64, 
    withdrawalsts: String
    makerfee:u64,
    takerfee:u64,
    misc: String   
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
    origQty: u64, 
    execQty: u64, 
    activeTime: Duration, 
    Ordertype: String, 
    action: OrderSide,
    time: i64,
    fiils: fills
}

// Structure of user balance 
struct Balance
{
    symbol: String,
    name: String, 
    qty: u64,
    availqty: u64,
    totval: u64
}
 
// Structure of OrderBook 
struct OrderBook
{
    lastupdateId: u64,
    type: OrderType
}
    
struct OrderType
{
    ordType: Type, 
    price:[u64,1000],
    qty:[u64,1000]
}
    
    
    
    




    
