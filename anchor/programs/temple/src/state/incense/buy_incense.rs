use anchor_lang::prelude::*;
 
 

/// 单个香型的购买项
#[derive(Debug, AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct BuyIncenseItem {
    /// 香的类型ID
    pub incense_type_id: u8,
    /// 购买数量 (1-10根)
    pub quantity: u8,
    /// 单价 (lamports)
    pub unit_price: u64,
    /// 小计金额 (lamports)
    pub subtotal: u64,
}

impl BuyIncenseItem {
    pub fn validate(&self) -> Result<()> {
        require!(
            self.quantity >= 1 && self.quantity <= 10,
            BuyIncenseError::InvalidQuantity,
        );

        require!(self.unit_price > 0, BuyIncenseError::InvalidPrice,);

        require!(
            // 检查小计金额是否正确 checked_mul用法防止溢出
            self.subtotal == self.unit_price.checked_mul(self.quantity as u64).unwrap(),
            BuyIncenseError::InvalidSubtotal,
        );

        Ok(())
    }
}

/// 购买相关错误定义
#[error_code]
pub enum BuyIncenseError {


    #[msg("Invalid treasury account")]
    InvalidTreasury,

    #[msg("Invalid buy incense quantity ")]
    InvalidQuantity,

    #[msg("Invalid price, must be greater than 0")]
    InvalidPrice,

    #[msg("Invalid subtotal calculation")]
    InvalidSubtotal,

    #[msg("Too many buy incense items, maximum is 6")]
    TooManyBuyIncenseItems,

    #[msg("Empty buy incense list")]
    EmptyBuyIncenseList,

    #[msg("Insufficient payment amount")]
    InsufficientPayment,

    #[msg("Payment amount mismatch")]
    PaymentMismatch,

    #[msg("Incense type not available for buy incense")]
    IncenseTypeNotAvailable,


    #[msg("Invalid incense type")]
    InvalidIncenseType,
}
