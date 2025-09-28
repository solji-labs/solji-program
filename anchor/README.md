
## 指令清单
### 1. initialize — 初始化香配置（管理员）
+ 用途：创建/初始化 `incense_rules_config`。
+ PDA：
    - incense_rules_config
        *  seeds = [b"incense_rules_config"]

### 2. update_incense — 修改香的规则（管理员）
+ 用途：更新某一类香的规则。

### 3. create_temple — 创建寺庙（管理员）
+ 用途：初始化全局寺庙账户。
+ PDA：
    - temple
        * seeds = [b"temple"]

### 4. create_user — 首次使用(烧香,抽签,许愿...)前创建用户信息
+ 用途：为钱包初始化 `user_info`（记录香、功德、抽签次数等）。
+ PDA：
    - user_info
        * seeds = [b"user_info",authority.key().as_ref()]

### 5. incense_buy — 购买香
+ 用途：按类型与数量扣费 & 增加用户香的持有数/记录。

### 6. incense_burn — 烧香
+ 用途：根据规则燃烧香，可能触发 NFT 铸造、更新功德与计数。
+ PDA:
    - nft_mint_account
        *  seeds = [b"create_burn_token",authority.key().as_ref(),args.name.as_bytes()]

### 7.initialize_lottery_poetry — 初始化签文（管理员）
+ 用途：写入签文数组到 `lottery_array`，并初始化 `player_state`。
+ PDA: 
    - lottery_array
        *  seeds = [b"lottery_array"]
    - player_state
        *  seeds = [b"playerState".as_ref(), authority.key().as_ref()]
  
+ 抽签前初始化

### 8. coin_flip — 随机数前置指令
+ 用途：提交/记录用于后续抽签的随机性（例如记 slot 和种子）。
+ randomness_account_data
    - 这个指令devnet上一直超时,没办法测试,下面是获取代码;详细见[链接](https://docs.switchboard.xyz/product-documentation/randomness/tutorials/solana-svm)

```rust
export async function coinFlip(
  wallet: anchor.Wallet) {
  const connection = new Connection(
    "https://devnet.helius-rpc.com/?api-key=自己的key",
    { commitment: "confirmed" }
  );

  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const sbProgramId = ON_DEMAND_DEVNET_PID;       // Devnet 上的随机数程序
  const sbQueue = ON_DEMAND_DEVNET_QUEUE;        // Devnet 队列地址
  const sbProgram = await anchor.Program.at(sbProgramId, provider);
  const queueAccount = new Queue(sbProgram, sbQueue);
  const rngKp = Keypair.generate();
  const [randomness, ixCreate] = await Randomness.create(sbProgram, rngKp, sbQueue);
  console.log("Randomness:", randomness.toString());
  await provider.sendAndConfirm(new Transaction().add(ixCreate), [rngKp])

  const commitIx = await randomness.commitIx(sbQueue);
  console.log("Commit Ix:", commitIx);
  await provider.sendAndConfirm(new Transaction().add(commitIx));

  return await program.methods.coinFlip()
    .accounts({
      randomnessAccountData: randomness.pubkey,  // 作为 AccountInfo 传进合约
    })
    .rpc();
}

```

### 9. draw_lots — 抽签
+ 用途：执行抽签逻辑（扣除费用、随机选签、更新功德、写入记录等）。
+ coin_flip这个指令在devnet上没使用,逻辑里面随机数在主网上才会调用预言机
+ PDA: 
    - lottery_record
        *  seeds = [b"lottery_record",authority.key().as_ref(),(user_info.lottery_count+1).to_string().as_bytes()]

```rust
export async function drawLots(wallet: anchor.Wallet) {
  let r1 = await program.account.userInfo.fetch(getUserBurnInfo(wallet));
  let pda = getLotteryRecordPda(r1.lotteryCount, wallet);
  let [playerPda] = getPlayerPda(wallet);
  const st = await program.account.playerState.fetch(playerPda);
  let r = await program.methods.drawLots()
    .accounts({
      lotteryRecord: pda,
      randomnessAccountData: st.randomnessAccount,
    })
    .rpc();

  let r2 = await program.account.lotteryRecord.fetch(pda);
  return [r, r1, r2];
}
```

### 10. create_donate_count / create_donate_record — 捐助（及计数器）
+ 用途：先创建计数器 `donate_count`，再捐助生成 `donate_record`，并将款项/功德计入。
+ create_donate_count 这个指令在create_donate_record前(每个用户)初始化一次
+ PDA:  
    - donate_count 
        *  seeds = [b"donate_count", authority.key().as_ref()]
    - donate_record 
        * seeds = [b"donate_record", authority.key().as_ref(),(donate_count.count+1).to_string().as_bytes()]
            + count是donate_count中捐助的次数

### 11. mint_sbt_nft — 铸造 SBT
+ 用途：按名称为用户铸造不可转移的徽章（SBT）。
+ PDA：
    - sbt_nft_mint_account
        * seeds = [b"create_sbt_token",authority.key().as_ref(), args.name.as_bytes()],

### 12. destroy — 销毁 NFT
+ 用途：销毁指定 `nft_mint_account`(烧香获得) 下用户持有的 NFT。

### 13. create_like — 点赞愿望
+ 用途：对某条 `publish_wish` 点赞；同一用户对同一愿望通常只能有一条 `wish_like`。
+ PDA：
    - wish_like
        * seeds = [b"wish_like",user_info.key().as_ref(),publish_wish.key().as_ref()]

### 14. create_wish — 许愿
+ 用途：创建一条愿望，支持匿名。
+ PDA:
    - publish_wish
        * seeds = [b"publish_wish",user_info.key().as_ref(),(user_info.wish_total_count+1).to_string().as_bytes()]
            + wish_total_count是 user_info中许愿的数量

### 15. withdraw — 提现（管理员）
+ 用途：从 `temple` 提取 SOL 到管理员。

## 说明
+ IDL 里 `nft_mint` 被标注为废弃
+ devnet环境没有初始化任何PDA

