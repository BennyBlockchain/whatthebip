
use crate::bip::{BipItem, Resource};

pub fn get_bips() -> Vec<BipItem> {
	let bip_list = vec![
		BipItem {
			name: "BIP32".to_string(),
			resources: vec![
				Resource {
					title: "Twitter thread".to_string(),
					site: "Twitter".to_string(),
					link: "https://twitter.com".to_string(),
				},
				Resource {
					title: "Youtube".to_string(),
					site: "Youtube".to_string(),
					link: "https://youtube.com".to_string()
				}
			]
		},
		BipItem {
			name: "BIP39".to_string(),
			resources: vec![
				Resource {
					title: "Twitter thread".to_string(),
					site: "Twitter".to_string(),
					link: "https://twitter.com".to_string(),
				},
				Resource {
					title: "Reddit thread".to_string(),
					site: "Reddit".to_string(),
					link: "https://google.com".to_string()
				}
			]
		}
	];
	bip_list
}
				
