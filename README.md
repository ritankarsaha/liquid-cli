## Proof of Work -- 

To set up the repository properly for working follow the following steps --

```bash
git clone https://github.com/ritankarsaha/liquid-cli.git
cd liquid-cli
cargo build
```
The `cargo build` command will succesfully build your rust project and now it is up for running :)

You will be getting output something like this -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```

Now let's take the project for a proper spin. 

Oh, first make sure you have elements core first installed on your machine.

The installation steps may be different depending on your operating systems, so I am not exclusively adding any steps for this, however if you face any problem you can always raise an issue for that and I will be sure to include the steps as well.

I have installed the same on Ubuntu Linux and MacOS

I am using an Ubuntu Linux fo taking the project for a spin now :) 

Ok, after the installation start elements core with this command-- 
Source Directory will be varying. 

```bash
~/Downloads/elements-23.2.7/bin/elementsd -conf=$HOME/.elements/elements-testnet.conf
```
Make sure you are getting an output saying `Elements Core starting`

After that to get the proper blockchain info run this command -- 

```bash
~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-testnet.conf getblockchaininfo
```

The output will be something like this -- 
```bash
{
  "chain": "liquidregtest",
  "blocks": 204,
  "headers": 204,
  "bestblockhash": "e68243c127a84a0d4dc720fa0764adf641a728313c8f42e52b148f8e50b01f7a",
  "time": 1745274831,
  "mediantime": 1745274673,
  "verificationprogress": 0.346,
  "initialblockdownload": false,
  "size_on_disk": 88485,
  "pruned": false,
  "trim_headers": false,
  "current_params_root": "3700bdb2975ff8e0dadaaba2b33857b0ca2610c950a92b1db725025e3647a8e1",
  "current_signblock_asm": "0 4ae81572f06e1b88fd5ced7a1a000945432e83e1551e6f721ee9c00b8cc33260",
  "current_signblock_hex": "00204ae81572f06e1b88fd5ced7a1a000945432e83e1551e6f721ee9c00b8cc33260",
  "max_block_witness": 74,
  "current_fedpeg_program": "a91472c44f957fc011d97e3406667dca5b1c930c402687",
  "current_fedpeg_script": "51",
  "extension_space": [
    "02fcba7ecf41bc7e1be4ee122d9d22e3333671eb0a3a87b5cdf099d59874e1940f02fcba7ecf41bc7e1be4ee122d9d22e3333671eb0a3a87b5cdf099d59874e1940f"
  ],
  "epoch_length": 10,
  "total_valid_epochs": 2,
  "epoch_age": 4,
  "warnings": ""
}
```

If you are connected to the main liquidv1 chain of the blockchain you will be getting outputs more like this-- 
NOTE -- (This is out of the scope of the competency test)
```bash
./elementsd -printtoconsole
```
Output Logs Screenshot --

<img width="1368" alt="Screenshot 2025-04-25 at 4 51 22 AM" src="https://github.com/user-attachments/assets/66861ab4-1b8f-40c0-ab85-c17440465e6f" />
<img width="1360" alt="Screenshot 2025-04-25 at 4 52 04 AM" src="https://github.com/user-attachments/assets/57b4be65-a9fe-4e90-9af3-7c8d58cef25d" />

And on getting the blockchain info, the outputs would be looking something like this -- 

```bash
elements-cli getblockchaininfo
```

Output Logs Screenshot --
<img width="1367" alt="Screenshot 2025-04-25 at 4 54 27 AM" src="https://github.com/user-attachments/assets/4534dbc4-8824-44af-b109-672cf6a57141" />
<img width="1461" alt="Screenshot 2025-04-25 at 4 54 48 AM" src="https://github.com/user-attachments/assets/22f98002-6f7d-498a-a0f1-e737cdd361d2" />

Now to create a new wallet and load it follow the underlying command -- 

```bash
cargo run -- wallet --create ritankar
```

We will be getting an output somewaht like this showing that the wallet has been properly loaded onto the system -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- wallet --create ritankar
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/liquid-cli wallet --create ritankar`
Successfully created and loaded wallet: ritankar
```
![image](https://github.com/user-attachments/assets/ce9312c5-ba92-44ae-8ace-1f4e9f2168bd)

Now to generate a new liquid address follow the underlying command -- 

```bash
cargo run -- address --new
```
We will be getting an output somewaht like this -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- address --new
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/liquid-cli address --new`
New Liquid address: el1qqdrvz9sa8z6htyz468adc4vd7c8xxwaetvpnar4y9zyld30typqm5lygv2y9ey7wgl48v4dz6k678rpvwjf5jdcsl3uqrsqp0
```
Now, we will be generating a second liquid address using the underlying command -- 

```bash
cargo run -- address --new
```
We will be again getting a similar output for that. I am just adding the logs for that to keep this readme precise and crisp :)

```bash
itankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- address --new
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/liquid-cli address --new`
New Liquid address: el1qqdung5eemndaujmjsl4slh54vknwyjsz2p0ty5sfzsxm8zcjrvp85npchz6mxe6sm9pw6uzcqq2pvqjzms375md3uhtn5g2zu
```
You can see that another liquid address has been created properly 

Now, to list all the liquid addresses generated for the wallet run this command -- 

```bash
cargo run -- address
```
We will be getting an output having the list of all the addresses like this -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- address
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/liquid-cli address`
Existing Liquid addresses:
  el1qq2h6nfhm87yx2nal9ja8pqs8wz5ksruy4cu4582m3424zfu74cxck9r9ny02ahu7y2l5ck3yxshlmshgttuazgpk9uh20a8qp
  el1qqdrvz9sa8z6htyz468adc4vd7c8xxwaetvpnar4y9zyld30typqm5lygv2y9ey7wgl48v4dz6k678rpvwjf5jdcsl3uqrsqp0
  el1qqdung5eemndaujmjsl4slh54vknwyjsz2p0ty5sfzsxm8zcjrvp85npchz6mxe6sm9pw6uzcqq2pvqjzms375md3uhtn5g2zu
  el1qqdw5fxr5xgypk4yu5uu8eme4vya7ukjx0kqe4r5cv4jlpnmzax8xc9fmqmwcv0wpwntvw32zkhdg8fhjr3qd60tgm86ev8mwk
  el1qqgu7g4nmr4s486ldh63svrufwtd2p57z5paznz5lykuqjt6404zhwyh36vnp3f94aggnzmmk6g98t0x4zmnrw83msehmdd2tl
  el1qqtemc3d8wnhjt4k8zfn0erfpyy98uhlev2n4zmfchz47pledupe57wrl23up8fn3eyjg0pd0zz9cclu9smlr2p6y46ukes4nu
  el1qqtl9er644uvcpzf4er3us79xm23ajtepva0qh9uupeuyflfsqhpk96qpy3m7yec4vc0ew9lhmp69cgugutq9yp83vh7vqpa7m
  el1qqv425dhzfvw7zqqstawq4vdjerzpr2y9tesrwfg42wak3hhy22dgsagyjkd59w67r7y7z7ly7zxy3klfm4mcj5l7399x89979
  el1qqvuruegq3zzc4sh8stjevx0mc7ennxh4j9uvx0equa4nh73z84a8k5cl5qq2qjf52kw8wpukvdy5lmugww7rv2mxn8zhqfwak
  el1qqwaj397uev7aj8zgx95a0l56zet74s4gfzc9prpqkp3ndagsr4yp50yaqq9kzu6f8j2ee24jeptmp22hf48yslve6mzlf45dy
  el1qqwqc8z5skng04ca6j9hlyt6xf2q02dn94qevtlqvku87cs4yf9etwlz76zj0q0me5g5knpwxjqsncf2p6g0wq8zwkp3lagmmg
  ert1q63e3h2mtcpq4kvavplqa7pwa8l20h24npyjfkf
  ert1qs62ny0nlr9k4899m7r38wqq49pgfrzvagjwqgp
```
![image](https://github.com/user-attachments/assets/31a6df7a-26b3-4c1b-bc2f-8b1e309c7ec6)

Actually I had taken the project for a spin earlier as well checking the functionalities and all so you can see a list of addresses but for your case if that was the first time for you generating the addresses inside the wallet, you will be getting twi liquid addresses only. 

Now, to check asset information run this command -- 

```bash
cargo run -- asset
```
Output -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- asset
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/liquid-cli asset`
Your Liquid Assets:
  Asset: bitcoin
    Balance: 0.0
    Name: Unknown
    Precision: Unknown
```

You can also generate blocks with mining rewards with this command --
The directory names will vary .

```bash
NEW_ADDRESS=$(~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet getnewaddress)
~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet generatetoaddress 101 "$NEW_ADDRESS"
~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet getbalance
``` 

Output with the blocks mined -- 

```bash
[
  "2a0ce56320c86b001996dd5a84394469eb228627a2cc290d02be20f6c5497824",
  "df42545d958f937dde8a2014b7ad972e9947ebc09b430a24035bd84c5f8a222b",
  "853a5b98d005de39bddcb9174d454b6393660dc964cc015e54786e5316c4cbea",
  "edef732934fa7e1e2fc6d5d027ccd19fec762df3730248836b1e2e51dae6cea4",
  "ca908a4066e703c6f57b8396c10cc05246869d4f810c417450b63d51ad149397",
  "241756772d92d4161684dd602f73f64a638dd96d58eb31af98cf61dff12ddb34",
  "17915fde9ef045dee300faee758c1d505843e826cac540e155ec01480d2ea688",
  "35c67f2aaafa3e0c3bb7c933d4ca2488bc2b5ef5053e6a59437020d2f66aef36",
  "f3ad37c84688bbdf1e9f71fb18e8e29604d0fef7ebf93fd60f6268d3c85caaca",
  "dee530add6c1ac0b931d9cb0dd2256f46a0d3f710977f6e748ba2ed5022a3911",
  "0326f169c78fd37658149508e7497e1764849a4d1bfb7b8ead89355051b3bd78",
  "351e76b51fbd3aed2b35ef6c3a746a3d3dc2691c6cb1f54dc9709a61303ee042",
  "b2322e86a14a89dbb59c1698503e0e94277cea4fcd5db0bfa262fb905e14089f",
  "812293f44d3d8e6b76d355b2a57439af3414b0c4080061169ebc8cff1d473c88",
  "270f9b9d9638f9c1c978164332f2fc2fa4d757a75943c3f44a993110753eb2c9",
  "a2a64c2a9a1d79aed500cb7e4c688d0e5ac58ad0e157a952827314cfeaa05337",
  "a569c1edbef6655bcef9b7cb0071a3a4d652dd9db45cc398045e456223496aa9",
  "a4bfac786e19a8cd3b97ceca3bdf094566315f2bed2916df872340be48b786c4",
  "6ac7e3e55bafd1cb5696cbefe5e6bb7bde60e78ec63b578e7031dc3cdeefb476",
  "a0cda0d9b9e85a5b5c0c5ffba6f0059a3d1048827116a49fdf995071034f49a7",
  "390ff78c9ad9478fb18412e30b8bb263e07cc3bfc25c17f09da602c49b7676cd",
  "6098c90b3d2995188deb1990290c526962c6f3922abcc68fb96188099dbd4a4e",
  "e47d226a31507bc55f090fd7c4d31cc835de083ccb959a819531dd792d33516c",
  "c489de31d062789869c2560820382cf51adb97e1bea76dea8380c06655d32acf",
  "08ab00752099e7583e3ff85c2e1d190c0a52492741195b55c03f81dd26f2f133",
  "d485a9faa98aa044cb6b734bd9ec5eb06ace34c2d9335124de6424675329f955",
  "5a753b81be742b48d8dce2d62a81b4d88ba98797760318790b6f61ee4092af5b",
  "7944f674a0413c82c5127d7e09b7ec8571c41670dc2a9e5f03ea175fa8b8a624",
  "167971f50153a7d21ed37e5df08c495020de53f44013b0d0d71ebc9461df7303",
  "adfbfc817f335117a301ffdc3e4642a9eed51c94f682b77938f3d81285cc1e75",
  "4befee462c92a6006131f5139afe68bbba1dbc86c4dde3c18da6027cf6b93b59",
  "233b2906cc85462860c5f53573eba0e534f478b237290e9e198d6db720dc4637",
  "cd476239f62c4eb5ced4a3652afd8dd003787d1c28fcf3f4f2402f1b677d27ec",
  "5a9a865a58cfc1a70e2b636a879c739191f40293bbfbdba4229bc20bc799025c",
  "7525c1686237258b90848f16a45bc26a54a736d5db84a13aec9f858ee969d9af",
  "c70fdc5088291aee09b75b857ecae01722ca7449c2b6fa8b0f5e92b0a52e6167",
  "73cffb2e78a7f7eb724b20dc60a1deecae3bdbf10fd5ce817fdfcc086a2e9592",
  "51d2f175613534ea0a1fcc9dd19850cb5e53ae62dedba01c691d74d40dda6c6f",
  "3b4997c70aede601e3126bffa1dc37e12d0ae17d642bcc87b45a4214cdc51c7a",
  "1615341c2d659e570eb7d82c2a15bb537d307fa07e88f7fd0018e2aa0751c204",
  "5e5dd752e116a4a1f0dc0c175dd8dd66d065c68567a77711c235a8adb228007a",
  "d119283830bc03651e3167e0955a4d6cdcbcab41a7ea3a9346aaffdf6ca7d7a2",
  "a2b8b1a038991bca4c2e258d0b2216a0d544fc0ca55a094bd0e823459b8feb7c",
  "72bb2817f70d21cbc76a8e72166924e79b31e5bf56f4fb2c33408ba27482f25e",
  "1f010c890b3cb2a43b00e3d80bca5fe859c4a8aff448f9b610bb13033525e2a8",
  "00308e4937bfedcaa853cf1e195e7444ec70f9eaee027cf9c9dd80e4f0a73cdb",
  "9c2c118b57d0e21821feb7fe1b2d24821c00db47b371bbe2650c61183b85684c",
  "329c0ffaed69638a2514755fe73ff3e56301e4327edaf2de83ebdc7d238e2180",
  "32624aa5b59a8a2459ccbc644038d9491f6ef6bd285f3c8ff584c00db27af5df",
  "6733280b43d2a3a37fe46f78d872bb3a4b28b5f9e340d1b1c900db70671a26fe",
  "2760af7197a8564091c4c22a00e89f092ed05b98d5d5d9d10c36e02bdefab224",
  "cc8130105cb32a19044c9229b5fbcc5febcfdc86da4d5549ade79e0c4fee3dc8",
  "de544eb0891f06079f8eb0931f70e6d55bce6057c0606d51251cbac8ee98936c",
  "5b8b7d92a08bae0e438129ea7e6e0dce37bf4f0622551046656f1ed64c4131eb",
  "3d148915a26d18063120674ebf603f7a4b68fb4aa8f9bc8b1c098a076b110ad8",
  "afa556d6311b275cc92a7be09f895f6c52f9363d04b0fbf7fbaf21d0cda75b0e",
  "a317ea1c368ab3880574a3c82c74149a4e8c91b8f3d9d8ac83e8341667391d32",
  "4f2b51111f5761be975df386ff8f50a962d7bbbcffa55e9a2b4ece99d35801a1",
  "230db32222e1b6a329070f1ea9176a077100165408dea223b20548cd81da743b",
  "ada41aebae0e38e903a44257efe7dfb0abd78887dbd0ab6d8e49f255a69afa16",
  "c8db885cdb9ecf3b808765b80bbf33ec5be1832c280965334db1a35ccd421083",
  "4413547ab8869610cbb9343ad30086863d2498e11613aec8262904006fed5242",
  "004e25e093db276f2ef8531bd60441e8e1a6e27bb989c2274c431a58e4a28211",
  "559abfc0519fdfa62e6f4c0b8597458f9c9d2112cf70ca06b23f672834075688",
  "6d2b2532142099d0a22416924b85903a20593882b7a28b26fa833b46d48fa5cb",
  "b91599496d308f2174e6a89016210e772e8aae0a830e5b126852cb17d073a379",
  "eaa6109e05b32db917474a2010f9ad4f64f4095239e407e95c740e3120e994e3",
  "23ab548c6b9b291cae0a0b3c3cb2008ac33abf99d67bc626e9abb6b7405fee52",
  "44f4b02082c09a54c5c9c02b1d5257f6d4d733a445e29cf793f553bc4eeaf903",
  "f40b0250ae0b794c4fda2b753b20e8519d80365e3f078f66830f7e4df5ba30bc",
  "03bf46e8d7bd2f2a102c8ce311d768da2d1aa534ba6d892f30abd99127e1470e",
  "10c53765f41879e520b902c31251093fb201d4b71fe9d57c858ab487b8b7169f",
  "7b88021ebf1bf1656226b70a48c46bb2dc8e163a7002f87a2914efb6a00d9d7c",
  "64a5962ce60f0174f193ff0ea5a6177be0a6753cd8cdf5ec2aa1c0041896025d",
  "a696036d4c3f7dc22ba744aaad2d0569a8fbb7cbc0975d00ef28c31b1033eb07",
  "50e961120435601e72899020e6a907bb4fbcf47d62bdac2e6e40f708bf35e3a5",
  "1fafb4616e07a77d59c49a0b07095c6d1806b88680f0b67fe0c03e0002bea26e",
  "8a8f61c86634d9890bd46fb16d04eb02a7c3d95c9996f2f39b5a293416d13de3",
  "5f61242f319dc53881cc39d180899852762d798d91df8ce1e6e6eb4dd1916dd2",
  "9278ab944b3b613a7c76741ffe1a9a850dbea6361dd6dcc0447077c9a8e725c8",
  "20ee071c08ff907699cd8b7f39b48a25697a500ef2dadb208277af410642ccef",
  "78230a0cd73381548390d2cc215b6844616b0423505613a116d8984ab03725d7",
  "1ba0b8f36464e2057a2104200a9115c4c3e9ee5d55c439d8cfd84c4effa5bc7b",
  "2b6b56d2448d7bf92268c374cb9b804481612fc03e5bc7280d6bb3cc6d3a598a",
  "ba9dab757fe58f570e4c7739e6db7d83c1d06aee01e201898a59d42f5a5022f8",
  "49281656876a9a46e6ef958a808bf1cc07bb4e602f0e4339d317b7ec6ab4e7d0",
  "c201afedc7e36adcc8bb7d7df8ba83f5599ceed87833960dcee8655f55afd3d9",
  "5ba59303e562358eff09dab57fa2ba735312f88f4d9620999d82f80117426590",
  "9666b763ce79487cfd9178e6297916527f1a40a303e3dc126c1a17befe55e9b5",
  "954123b27c164b2777ede0e99b276cb7e86a955ce95d9a9f448d3723b71c12af",
  "9e55c24a9e35052d6a79b1faa6bc80c03e2b186658fe4cd815ef22a43e3f4e19",
  "be9d8cf92d635d645eeab953bc1470906312fed69a7155364e5f0ee948e9fcd4",
  "524ef97038cc690f448fcc33be5ade740867cd459c2253be23c7b5b2c99a5459",
  "9be08367841f6f61bbae5e6b7e09caecc1c61363a5470d060ce454089376b62a",
  "181d5bc5a352f407a26333f43df2c884218bc2fae830070e4c5e6492ec266b80",
  "3bb1a768856569204ad6c986b84b1e84628335696cb8fbc7712f23a93aebb259",
  "45bc9eb67939e442793c136977d536eb9fb1adc26be6c3a29741b22586c94ae9",
  "5801ec2cd4c3c827a52fab97e357d3a0f5616b765455a7dc69df9e7b3137d4a6",
  "8416e5cf06afa7455b89a878b7b50fcace19ce7e68cabb7c7a9c353ae095eadf",
  "edb6b571d2936641658051f1c356b99efe82b97801c67d786eceacf16742605e",
  "62403bf822bbea584fc810820dbfa07d193d99ef736d9222297a5968ac87a411"
]
{
  "bitcoin": 0.00000000
}
```

Now, create another wallet named ritankar_wallet and run the following commands

```bash
ADDRESS1=$(~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet getnewaddress)
ADDRESS2=$(~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet getnewaddress)

echo "Address 1: $ADDRESS1"
echo "Address 2: $ADDRESS2"

# Get the bitcoin asset ID
BITCOIN_ASSET=$(~/Downloads/elements-23.2.7/bin/elements-cli -conf=$HOME/.elements/elements-regtest.conf -rpcwallet=ritankar_wallet dumpassetlabels | grep -o '"bitcoin": "[^"]*"' | cut -d'"' -f4)

echo "Bitcoin Asset ID: $BITCOIN_ASSET"
```

You will be getting an output somewhat like this 

```bash
Address 1: tlq1qqwadhkeygngantphrpu9fw70u73kcym2ww26zq676c900yhtpn4f4enedfq49c2m9eackhccm6z2zwahd0rcxvv5pgxp4rm9d
Address 2: tlq1qqfcxxsd6ng5pwdc3nvyhdt62ge0sgm34fs2kpdq5852pz6eam50xls09g55m50t95epkvvurdd7a0jxr324ezmq5h72dd9ukd
Bitcoin Asset ID: 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49
```

This shows that the bitcoin asset has been properly created.

Now, let's focus on the proof of concept  --- Demonstrating transferring an asset between two liquid addresses using the libraries that would be incorporated into Angor.

Ensure you have the wallet named `ritankar_wallet` created and loaded. If not follow the steps -- 

```bash
cargo run -- wallet --create ritankar_wallet
```

Now, run the following commands below -- 

```bash
 cargo run -- transfer-poc
```

We will be getting an output like -- 

```bash
ritankar-saha@ritankar-saha-Inspiron-15-3511:~/liquid-cli$ cargo run -- transfer-poc
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/liquid-cli transfer-poc`
Starting Liquid Asset Transfer Proof of Concept
-----------------------------------------------

Step 1: Generating two addresses
Address 1: tlq1qqw3agna4kvc6y85g3cly8d8xxruvwm6pu6kjap47234jydhsg76j0gpxreckwq7sp3yutrvy6naa8jcdsavzw5wl8gx02mev8
Address 2: tlq1qqv7s0ykdha229ju7hj4pkajhx9lzu6z24ymdrd5a3l3t0x3cjsf622nvezmygqzuz3pl9csdqkc6zr3rs9fz2myccn0qx66dw

Step 2: Checking for existing assets

Step 3: No assets with balance found. Attempting to issue a new asset...
Failed to issue asset: RPC error (-4): Insufficient funds
Will try to use the bitcoin asset for demonstration
Using bitcoin asset: 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49

Step 4: Transferring asset to the second address
Asset ID: 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49
From: tlq1qqw3agna4kvc6y85g3cly8d8xxruvwm6pu6kjap47234jydhsg76j0gpxreckwq7sp3yutrvy6naa8jcdsavzw5wl8gx02mev8
To: tlq1qqv7s0ykdha229ju7hj4pkajhx9lzu6z24ymdrd5a3l3t0x3cjsf622nvezmygqzuz3pl9csdqkc6zr3rs9fz2myccn0qx66dw
Transfer failed: RPC error (-6): Insufficient funds

Alternative demonstration: Simulating a transfer
Simulated transfer of asset 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49 from tlq1qqw3agna4kvc6y85g3cly8d8xxruvwm6pu6kjap47234jydhsg76j0gpxreckwq7sp3yutrvy6naa8jcdsavzw5wl8gx02mev8 to tlq1qqv7s0ykdha229ju7hj4pkajhx9lzu6z24ymdrd5a3l3t0x3cjsf622nvezmygqzuz3pl9csdqkc6zr3rs9fz2myccn0qx66dw
Simulated transaction ID: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
```
![image](https://github.com/user-attachments/assets/00128db5-c8f3-463a-90b3-3e18d3d4d091)

Same can be done using the shell script I have created and added. Fix the permissions of the shell script properly first. Then run,

```bash
./liquid_transfer_poc.sh
```

You will be getting the following output

```bash
===== Liquid Asset Transfer Proof of Concept =====
-----------------------------------------------

Step 1: Generating two addresses
Address 1: tlq1qqtvrzkcpmm7dlwhfmxd5nvcf9wlae5d6yvslt2x6rqpeu88uv43ekyu97zh0ex9afnfnr2uh0l4dx82657cpj4wcv0s5wl4gy
Address 2: tlq1qqdlk38aax3lvq6vzkyuydpksrd9kcpwq9nuaevxg84sal7k5apve46n34fc40ewemtdqk257p0uxjgmq0gyzuuczuc8akpfe6

Step 2: Checking for existing assets
{
  "bitcoin": 0.00000000
}

Step 3: No assets with balance found. Attempting to issue a new asset...
Failed to issue asset: Insufficient funds
Will try to use the bitcoin asset for demonstration
Using bitcoin asset: 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49

Step 4: Transferring asset to the second address
Asset ID: 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49
From: tlq1qqtvrzkcpmm7dlwhfmxd5nvcf9wlae5d6yvslt2x6rqpeu88uv43ekyu97zh0ex9afnfnr2uh0l4dx82657cpj4wcv0s5wl4gy
To: tlq1qqdlk38aax3lvq6vzkyuydpksrd9kcpwq9nuaevxg84sal7k5apve46n34fc40ewemtdqk257p0uxjgmq0gyzuuczuc8akpfe6
Transfer failed: 
This is expected because the wallet has insufficient funds at present

Alternative demonstration: Simulating a transfer
Simulated transfer of asset 144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49 from tlq1qqtvrzkcpmm7dlwhfmxd5nvcf9wlae5d6yvslt2x6rqpeu88uv43ekyu97zh0ex9afnfnr2uh0l4dx82657cpj4wcv0s5wl4gy to tlq1qqdlk38aax3lvq6vzkyuydpksrd9kcpwq9nuaevxg84sal7k5apve46n34fc40ewemtdqk257p0uxjgmq0gyzuuczuc8akpfe6
Simulated transaction ID: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
```

NOTE - The transfer of assets from one liquid address to another liquid address fails, and the asset creation also fails as well bacasue the wallet has insufficient funds and this is quite expected. However we have used the `bitcoin asset` over here for a mere demonstartion purpose. The logic and the proof of concept is all there in the code.















