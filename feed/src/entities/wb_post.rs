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
    pub pid: u64,
    pub uid: u32,
    pub content: String,
    pub status: u8,
    pub created_time: u32,
    pub updated_time: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Pid,
    Uid,
    Content,
    Status,
    CreatedTime,
    UpdatedTime
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Pid,
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
            Self::Pid => ColumnType::BigUnsigned.def(),
            Self::Uid => ColumnType::Unsigned.def(),
            Self::Content => ColumnType::String(None).def(),
            Self::Status => ColumnType::TinyUnsigned.def(),
            Self::CreatedTime => ColumnType::Unsigned.def(),
            Self::UpdatedTime => ColumnType::Unsigned.def(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}