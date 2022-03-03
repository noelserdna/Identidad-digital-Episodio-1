//CAPAS trustoverip.org, https://identity.foundation/faq/
//1. CRUD Credencial descentralizada (red)
//2. Wallet para gestión de credenciales y comunicación entre wallets
//3. Roles: emisor, poseedor y verificador
//4. ecosistemas de identidad

// CASOS DE USO
// Comprar alcohol, soy mayor de edad?


use scrypto::prelude::*;

blueprint! {
    struct Adulto {
        adulto_def: ResourceDef,
        adulto_badge: Vault,
        admin_def: ResourceDef
    }

    impl Adulto {
       
        pub fn new() -> (Component, Bucket) {

            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Admin Adulto")
                .initial_supply_fungible(1);
           
            let adulto_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Badge Nft")
                .initial_supply_fungible(1);

            let adulto_def: ResourceDef = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Credencial de Adulto")
                .flags(MINTABLE)
                .badge(adulto_badge.resource_def(), MAY_MINT)
                .no_initial_supply();

            let comp = Self {
                adulto_badge: Vault::with_bucket(adulto_badge),
                admin_def: admin_badge.resource_def(),
                adulto_def
            }
            .instantiate();

            (comp, admin_badge)
        }

        #[auth(admin_def)]
        pub fn emisor(&mut self) -> Bucket {
            let adulto: Bucket = self.adulto_badge.authorize(|auth| {
                self.adulto_def.mint(1,auth)
            });

            adulto
        }

        pub fn verificador1(&self, adulto: BucketRef)  -> bool {
            assert_eq!(adulto.resource_def(), self.adulto_def, "Tu acreditación no es válida");
            assert!(adulto.amount() == Decimal::one(), "No posees verificación");
            info!("Es mayor de edad");
            true
        } 

        #[auth(adulto_def)]
        pub fn verificador2(&self) -> bool {
            true
        } 

        pub fn verificador3(&self, adulto: Bucket) -> (Bucket, bool) {
            assert_eq!(adulto.resource_def(), self.adulto_def, "Tu acreditación no es válida");
            assert!(adulto.amount() == Decimal::one(), "No posees verificación");
            (adulto,true)
        }

    }
}