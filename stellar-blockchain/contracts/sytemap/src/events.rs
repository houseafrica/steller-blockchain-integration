use soroban_sdk::{Address, Env, String, Symbol};

pub struct SytemapRegistryEvents {}

impl SytemapRegistryEvents {
    /// Emitted when a property info is minted
    /// - topics - `["new_property_info_added", plot_no: u32]`
    /// - data - `[plot_no: String, property_verification_no: String, token_id: u32, timestamp: u64, price_of_plot: u64]`
    pub fn property_info_created(
        env: &Env,
        plot_no: String,
        property_verification_no: u64,
        token_id: u64,
        timestamp: u64,
        price_of_plot: u64,
        buyer_wallet_id: Address,
        token_url: String,
        estate_name: String,
        size_of_plot: String,
        plot_url: String,
        date_of_allocation: String,
        coordinate_of_plot: String,
        estate_company_name: String,
    ) {
        let topics = (Symbol::new(&env, "new_property_info_added"), plot_no.clone());

        env.events().publish(
            topics,
            (
            plot_no,
            property_verification_no,
            token_id,
            timestamp,
            price_of_plot,
            buyer_wallet_id,
            token_url,
            estate_name,
            size_of_plot,
            plot_url,
            date_of_allocation,
            coordinate_of_plot,
            estate_company_name,
        ));
    }

    /// Emitted when a property price is changed
    ///
    /// - topics - `["property_info_price_changed", property_verification_no: u32]`
    /// - data - (["owner:Address",property_verification_no:u64,  ])
    pub fn property_price_changed(env: &Env, owner:Address, property_verification_no: u64, new_price: u64)  {
        let topics = (Symbol::new(&env, "property_info_price_changed"), property_verification_no.clone());

    env.events()
        .publish(topics, (owner, property_verification_no, new_price));
    }


pub fn emit_base_uri_updated_event(
    env: &Env,
    old_base_uri: String,
    new_base_uri: String,
) {
    let topics = (Symbol::new(&env, "base_URI_updated"), new_base_uri.clone());

    env.events()
        .publish(topics, (old_base_uri, new_base_uri));
}

}