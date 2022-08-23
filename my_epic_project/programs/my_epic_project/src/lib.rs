use anchor_lang::prelude::*; 
// 导入Anchor所提供的一些工具包

// declare_id主要用于向Solana提供如何运行我们所编写的程序，这块内容将会由Anchor为我们自动生成。
// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("49nDNU9kGM21q3WWjdm7ZTD7FBMfVWZAByRKMnp8micZ");

// 宏
#[program]
pub mod myepicproject {  // 定义了一个名为myepicproject的模块
    use super::*;
    // 模块中定义了一个名为start_stuff_off函数
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {  // 传入一个Context参数类型的同时输出返回一个Result结果
        // 当合约函数被外部进行调用时，通过访问新建合约帐户中的total_gifs变量进行初始化赋值。
        // 添加计数功能
        let base_account = &mut ctx.accounts.base_account;
        // // 初始化计数
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
// 定义函数上下文参数结构体，并且针对存储在新建帐户中的数据存储结构体进行初始化工作；
#[derive(Accounts)]
pub struct StartStuffOff <'info>{
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 定义存储用户提交的图片地址以及图片所在公钥地址信息struct
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct{
    pub gif_link: String,  // 储存图片地址
    pub user_address: Pubkey,  // 储存公钥地址
}

// Tell Solana what we want to store on this account.
// 定义数据结构体，用于记录总图片数据；
#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
