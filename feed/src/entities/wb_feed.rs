use arraystring::{typenum::U32, ArrayString};
use sea_orm::entity::prelude::*;

pub type TableName = ArrayString<U32>;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity {
    pub table_name: TableName,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        self.table_name.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub fid: u64,
    pub uid: u32,
    pub pid: u64,
    pub created_time: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Fid,
    Uid,
    Pid,
    CreatedTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Fid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = u64;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Fid => ColumnType::BigUnsigned.def(),
            Self::Uid => ColumnType::Unsigned.def(),
            Self::Pid => ColumnType::BigUnsigned.def(),
            Self::CreatedTime => ColumnType::Unsigned.def(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}