//! Generated by sea-orm-dbml 0.1.0-beta.2

pub mod bakery {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "bakery", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "String(None)")]
		pub name: String,
		#[sea_orm(column_type = "Double")]
		pub profit_margin: f64,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(has_many = "super::baker::Entity")]
		Baker,
		#[sea_orm(has_many = "super::cake::Entity")]
		Cake,
		#[sea_orm(has_many = "super::order::Entity")]
		Order,
	}

	impl Related<super::baker::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Baker.def()
		}
	}

	impl Related<super::cake::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Cake.def()
		}
	}

	impl Related<super::order::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Order.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod customer {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "customer", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "String(None)")]
		pub name: String,
		#[sea_orm(column_type = "Text", nullable)]
		pub notes: Option<String>,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(has_many = "super::order::Entity")]
		Order,
	}

	impl Related<super::order::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Order.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod baker {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "baker", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "String(None)")]
		pub name: String,
		#[sea_orm(column_type = "Json")]
		pub contact_details: Json,
		#[sea_orm(column_type = "Integer")]
		pub bakery_id: i32,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(belongs_to = "super::bakery::Entity", from = "Column::BakeryId", to = "super::bakery::Column::Id", on_delete = "Cascade", on_update = "Cascade")]
		Bakery,
		#[sea_orm(has_many = "super::cakes_bakers::Entity")]
		CakesBakers,
	}

	impl Related<super::bakery::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Bakery.def()
		}
	}

	impl Related<super::cakes_bakers::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::CakesBakers.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod cake {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "cake", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "String(None)")]
		pub name: String,
		#[sea_orm(column_type = "Decimal(Some((19, 4)))")]
		pub price: Decimal,
		#[sea_orm(column_type = "Integer")]
		pub bakery_id: i32,
		#[sea_orm(column_type = "Boolean")]
		pub gluten_free: bool,
		#[sea_orm(column_type = "Uuid")]
		pub serial: Uuid,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(belongs_to = "super::bakery::Entity", from = "Column::BakeryId", to = "super::bakery::Column::Id")]
		Bakery,
		#[sea_orm(has_many = "super::cakes_bakers::Entity")]
		CakesBakers,
		#[sea_orm(has_many = "super::lineitem::Entity")]
		Lineitem,
	}

	impl Related<super::bakery::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Bakery.def()
		}
	}

	impl Related<super::cakes_bakers::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::CakesBakers.def()
		}
	}

	impl Related<super::lineitem::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Lineitem.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod order {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "order", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "Decimal(Some((19, 4)))")]
		pub total: Decimal,
		#[sea_orm(column_type = "Integer")]
		pub bakery_id: i32,
		#[sea_orm(column_type = "Integer")]
		pub customer_id: i32,
		#[sea_orm(column_type = "DateTime")]
		pub placed_at: DateTime,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(belongs_to = "super::bakery::Entity", from = "Column::BakeryId", to = "super::bakery::Column::Id")]
		Bakery,
		#[sea_orm(belongs_to = "super::customer::Entity", from = "Column::CustomerId", to = "super::customer::Column::Id")]
		Customer,
		#[sea_orm(has_many = "super::lineitem::Entity")]
		Lineitem,
	}

	impl Related<super::bakery::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Bakery.def()
		}
	}

	impl Related<super::customer::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Customer.def()
		}
	}

	impl Related<super::lineitem::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Lineitem.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod cakes_bakers {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "cakes_bakers", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub cake_id: i32,
		#[sea_orm(column_type = "Integer", primary_key)]
		pub baker_id: i32,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(belongs_to = "super::cake::Entity", from = "Column::CakeId", to = "super::cake::Column::Id", on_delete = "Cascade", on_update = "Cascade")]
		Cake,
		#[sea_orm(belongs_to = "super::baker::Entity", from = "Column::BakerId", to = "super::baker::Column::Id", on_delete = "Cascade", on_update = "Cascade")]
		Baker,
	}

	impl Related<super::cake::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Cake.def()
		}
	}

	impl Related<super::baker::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Baker.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}

pub mod lineitem {
	use sea_orm::entity::prelude::*;

	#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
	#[sea_orm(table_name = "lineitem", schema_name = "public")]
	pub struct Model {
		#[sea_orm(column_type = "Integer", primary_key)]
		pub id: i32,
		#[sea_orm(column_type = "Decimal(Some((19, 4)))")]
		pub price: Decimal,
		#[sea_orm(column_type = "Integer")]
		pub quantity: i32,
		#[sea_orm(column_type = "Integer")]
		pub order_id: i32,
		#[sea_orm(column_type = "Integer")]
		pub cake_id: i32,
	}

	#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
	pub enum Relation {
		#[sea_orm(belongs_to = "super::order::Entity", from = "Column::OrderId", to = "super::order::Column::Id")]
		Order,
		#[sea_orm(belongs_to = "super::cake::Entity", from = "Column::CakeId", to = "super::cake::Column::Id")]
		Cake,
	}

	impl Related<super::order::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Order.def()
		}
	}

	impl Related<super::cake::Entity> for Entity {
		fn to() -> RelationDef {
			Relation::Cake.def()
		}
	}

	impl ActiveModelBehavior for ActiveModel {}
}
