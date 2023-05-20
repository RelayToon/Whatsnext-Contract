###################
# VoteController Add Community 
###################
# near call dev-1684618927800-74837450019059 add_community '{"community_id":"dev-1684588757249-57610888532698"}' --account_id test412ock.testnet
# near call dev-1684614398870-51564446730825 add_community '{"community_id":"dev-1684588863899-93178118256035"}' --account_id test412ock.testnet
# near call dev-1684614398870-51564446730825 add_community '{"community_id":"dev-1684588831975-64763231489253"}' --account_id test412ock.testnet
# near call dev-1684614398870-51564446730825 add_community '{"community_id":"dev-1684588808750-72116248629796"}' --account_id test412ock.testnet
# near call dev-1684614398870-51564446730825 add_community '{"community_id":"dev-1684567174380-54251358482328"}' --account_id test412ock.testnet


###################
# Community
##################
# near call dev-1684567174380-54251358482328 new_default_meta '{"owner_id": "cefa56b5a1a09e53729d55378bd1a469ea5ab11ed45c0ea2a9e0064d914395ef", "total_supply":"1000000000"}' --accountId test412ock.testnet

# near view dev-1684567174380-54251358482328 ft_balance_of '{"account_id":"cefa56b5a1a09e53729d55378bd1a469ea5ab11ed45c0ea2a9e0064d914395ef"}'

##################
# Vote Controller
##################
# near call dev-1684618927800-74837450019059 is_voting '{"community_id":"dev-1684588757249-57610888532698"}' --account_id test412ock.testnet

# near call dev-1684618927800-74837450019059 new_vote '{"prefix":"1","community_id":"dev-1684588757249-57610888532698"}' --account_id test412ock.testnet --gas 50000000000000

# near call dev-1684618927800-74837450019059 get_vote_account_id '{"community_id":"dev-1684588757249-57610888532698"}' --account_id test412ock.testnet


###################
# Vote
###################
# near call 

# near call dev-1684621900690-81323020463953 new '{"community_account_id":"dev-1684588757249-57610888532698", "vote_controller_account_id":"empty.empty"}' --account_id test412ock.testnet

# near call dev-1684614979842-20949303805930 add_proposal '{"title":"hello","prompt":"near","description":"now"}' --account_id test412ock.testnet

near view dev-1684621900690-81323020463953 get_votes_proposals

near view dev-1684621900690-81323020463953 get_all_proposals