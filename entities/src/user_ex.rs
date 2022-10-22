// pub mod org_account {
//     use crate::*;
//     #[sea_orm(table_name = "org_account")]
//     #[graphql(complex)]
//     #[graphql(name = "OrgAccount")]
//     pub struct Model {
//         #[sea_orm(primary_key)]
//         #[filter(use_alias = true)]
//         pub id: OpenDepartmentId,
//         #[filter(use_alias = true)]
//         pub account_id: AccountId,
//     }
//     #[automatically_derived]
//     impl ::core::clone::Clone for Model {
//         #[inline]
//         fn clone(&self) -> Model {
//             Model {
//                 id: ::core::clone::Clone::clone(&self.id),
//                 account_id: ::core::clone::Clone::clone(&self.account_id),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for Model {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::debug_struct_field2_finish(
//                 f,
//                 "Model",
//                 "id",
//                 &&self.id,
//                 "account_id",
//                 &&self.account_id,
//             )
//         }
//     }
//     impl ::core::marker::StructuralPartialEq for Model {}
//     #[automatically_derived]
//     impl ::core::cmp::PartialEq for Model {
//         #[inline]
//         fn eq(&self, other: &Model) -> bool {
//             self.id == other.id && self.account_id == other.account_id
//         }
//     }
//     pub enum Column {
//         Id,
//         AccountId,
//     }
//     #[automatically_derived]
//     impl ::core::marker::Copy for Column {}
//     #[automatically_derived]
//     impl ::core::clone::Clone for Column {
//         #[inline]
//         fn clone(&self) -> Column {
//             *self
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for Column {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             match self {
//                 Column::Id => ::core::fmt::Formatter::write_str(f, "Id"),
//                 Column::AccountId => ::core::fmt::Formatter::write_str(f, "AccountId"),
//             }
//         }
//     }
//     #[allow(missing_docs)]
//     pub struct ColumnIter {
//         idx: usize,
//         back_idx: usize,
//         marker: ::core::marker::PhantomData<()>,
//     }
//     impl ColumnIter {
//         fn get(&self, idx: usize) -> Option<Column> {
//             match idx {
//                 0usize => ::core::option::Option::Some(Column::Id),
//                 1usize => ::core::option::Option::Some(Column::AccountId),
//                 _ => ::core::option::Option::None,
//             }
//         }
//     }
//     impl sea_orm::strum::IntoEnumIterator for Column {
//         type Iterator = ColumnIter;
//         fn iter() -> ColumnIter {
//             ColumnIter {
//                 idx: 0,
//                 back_idx: 0,
//                 marker: ::core::marker::PhantomData,
//             }
//         }
//     }
//     impl Iterator for ColumnIter {
//         type Item = Column;
//         fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//             self.nth(0)
//         }
//         fn size_hint(&self) -> (usize, Option<usize>) {
//             let t = if self.idx + self.back_idx >= 2usize {
//                 0
//             } else {
//                 2usize - self.idx - self.back_idx
//             };
//             (t, Some(t))
//         }
//         fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
//             let idx = self.idx + n + 1;
//             if idx + self.back_idx > 2usize {
//                 self.idx = 2usize;
//                 None
//             } else {
//                 self.idx = idx;
//                 self.get(idx - 1)
//             }
//         }
//     }
//     impl ExactSizeIterator for ColumnIter {
//         fn len(&self) -> usize {
//             self.size_hint().0
//         }
//     }
//     impl DoubleEndedIterator for ColumnIter {
//         fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
//             let back_idx = self.back_idx + 1;
//             if self.idx + back_idx > 2usize {
//                 self.back_idx = 2usize;
//                 None
//             } else {
//                 self.back_idx = back_idx;
//                 self.get(2usize - self.back_idx)
//             }
//         }
//     }
//     impl Clone for ColumnIter {
//         fn clone(&self) -> ColumnIter {
//             ColumnIter {
//                 idx: self.idx,
//                 back_idx: self.back_idx,
//                 marker: self.marker.clone(),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl Column {
//         fn default_as_str(&self) -> &str {
//             match self {
//                 Self::Id => "id",
//                 Self::AccountId => "account_id",
//             }
//         }
//     }
//     #[automatically_derived]
//     impl std::str::FromStr for Column {
//         type Err = sea_orm::ColumnFromStrErr;
//         fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//             match s {
//                 "id" | "id" => Ok(Column::Id),
//                 "account_id" | "accountId" => Ok(Column::AccountId),
//                 _ => {
//                     Err(
//                         sea_orm::ColumnFromStrErr({
//                             let res = ::alloc::fmt::format(
//                                 ::core::fmt::Arguments::new_v1(
//                                     &["Failed to parse \'", "\' as `", "`"],
//                                     &match (&s, &"Column") {
//                                         args => {
//                                             [
//                                                 ::core::fmt::ArgumentV1::new_display(args.0),
//                                                 ::core::fmt::ArgumentV1::new_display(args.1),
//                                             ]
//                                         }
//                                     },
//                                 ),
//                             );
//                             res
//                         }),
//                     )
//                 }
//             }
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::Iden for Column {
//         fn unquoted(&self, s: &mut dyn std::fmt::Write) {
//             s.write_fmt(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_display(&self.as_str())],
//                     ),
//                 )
//                 .unwrap();
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::IdenStatic for Column {
//         fn as_str(&self) -> &str {
//             self.default_as_str()
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::prelude::ColumnTrait for Column {
//         type EntityName = Entity;
//         fn def(&self) -> sea_orm::prelude::ColumnDef {
//             match self {
//                 Self::Id => {
//                     std::convert::Into::<
//                         sea_orm::ColumnType,
//                     >::into(
//                             <OpenDepartmentId as sea_orm::sea_query::ValueType>::column_type(),
//                         )
//                         .def()
//                 }
//                 Self::AccountId => {
//                     std::convert::Into::<
//                         sea_orm::ColumnType,
//                     >::into(<AccountId as sea_orm::sea_query::ValueType>::column_type())
//                         .def()
//                 }
//             }
//         }
//     }
//     pub struct Entity;
//     #[automatically_derived]
//     impl ::core::marker::Copy for Entity {}
//     #[automatically_derived]
//     impl ::core::clone::Clone for Entity {
//         #[inline]
//         fn clone(&self) -> Entity {
//             *self
//         }
//     }
//     #[automatically_derived]
//     impl ::core::default::Default for Entity {
//         #[inline]
//         fn default() -> Entity {
//             Entity {}
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for Entity {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::write_str(f, "Entity")
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::entity::EntityTrait for Entity {
//         type Model = Model;
//         type Column = Column;
//         type PrimaryKey = PrimaryKey;
//         type Relation = Relation;
//     }
//     #[automatically_derived]
//     impl sea_orm::Iden for Entity {
//         fn unquoted(&self, s: &mut dyn std::fmt::Write) {
//             s.write_fmt(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_display(&self.as_str())],
//                     ),
//                 )
//                 .unwrap();
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::IdenStatic for Entity {
//         fn as_str(&self) -> &str {
//             <Self as sea_orm::EntityName>::table_name(self)
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::prelude::EntityName for Entity {
//         fn schema_name(&self) -> Option<&str> {
//             None
//         }
//         fn table_name(&self) -> &str {
//             "org_account"
//         }
//     }
//     pub enum PrimaryKey {
//         Id,
//     }
//     #[automatically_derived]
//     impl ::core::marker::Copy for PrimaryKey {}
//     #[automatically_derived]
//     impl ::core::clone::Clone for PrimaryKey {
//         #[inline]
//         fn clone(&self) -> PrimaryKey {
//             *self
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for PrimaryKey {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::write_str(f, "Id")
//         }
//     }
//     #[allow(missing_docs)]
//     pub struct PrimaryKeyIter {
//         idx: usize,
//         back_idx: usize,
//         marker: ::core::marker::PhantomData<()>,
//     }
//     impl PrimaryKeyIter {
//         fn get(&self, idx: usize) -> Option<PrimaryKey> {
//             match idx {
//                 0usize => ::core::option::Option::Some(PrimaryKey::Id),
//                 _ => ::core::option::Option::None,
//             }
//         }
//     }
//     impl sea_orm::strum::IntoEnumIterator for PrimaryKey {
//         type Iterator = PrimaryKeyIter;
//         fn iter() -> PrimaryKeyIter {
//             PrimaryKeyIter {
//                 idx: 0,
//                 back_idx: 0,
//                 marker: ::core::marker::PhantomData,
//             }
//         }
//     }
//     impl Iterator for PrimaryKeyIter {
//         type Item = PrimaryKey;
//         fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//             self.nth(0)
//         }
//         fn size_hint(&self) -> (usize, Option<usize>) {
//             let t = if self.idx + self.back_idx >= 1usize {
//                 0
//             } else {
//                 1usize - self.idx - self.back_idx
//             };
//             (t, Some(t))
//         }
//         fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
//             let idx = self.idx + n + 1;
//             if idx + self.back_idx > 1usize {
//                 self.idx = 1usize;
//                 None
//             } else {
//                 self.idx = idx;
//                 self.get(idx - 1)
//             }
//         }
//     }
//     impl ExactSizeIterator for PrimaryKeyIter {
//         fn len(&self) -> usize {
//             self.size_hint().0
//         }
//     }
//     impl DoubleEndedIterator for PrimaryKeyIter {
//         fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
//             let back_idx = self.back_idx + 1;
//             if self.idx + back_idx > 1usize {
//                 self.back_idx = 1usize;
//                 None
//             } else {
//                 self.back_idx = back_idx;
//                 self.get(1usize - self.back_idx)
//             }
//         }
//     }
//     impl Clone for PrimaryKeyIter {
//         fn clone(&self) -> PrimaryKeyIter {
//             PrimaryKeyIter {
//                 idx: self.idx,
//                 back_idx: self.back_idx,
//                 marker: self.marker.clone(),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::Iden for PrimaryKey {
//         fn unquoted(&self, s: &mut dyn std::fmt::Write) {
//             s.write_fmt(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_display(&self.as_str())],
//                     ),
//                 )
//                 .unwrap();
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::IdenStatic for PrimaryKey {
//         fn as_str(&self) -> &str {
//             match self {
//                 Self::Id => "id",
//             }
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::PrimaryKeyToColumn for PrimaryKey {
//         type Column = Column;
//         fn into_column(self) -> Self::Column {
//             match self {
//                 Self::Id => Self::Column::Id,
//             }
//         }
//         fn from_column(col: Self::Column) -> Option<Self> {
//             match col {
//                 Self::Column::Id => Some(Self::Id),
//                 _ => None,
//             }
//         }
//     }
//     #[automatically_derived]
//     impl PrimaryKeyTrait for PrimaryKey {
//         type ValueType = OpenDepartmentId;
//         fn auto_increment() -> bool {
//             true
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::FromQueryResult for Model {
//         fn from_query_result(
//             row: &sea_orm::QueryResult,
//             pre: &str,
//         ) -> std::result::Result<Self, sea_orm::DbErr> {
//             Ok(Self {
//                 id: row
//                     .try_get(
//                         pre,
//                         sea_orm::IdenStatic::as_str(
//                                 &<<Self as sea_orm::ModelTrait>::Entity as sea_orm::entity::EntityTrait>::Column::Id,
//                             )
//                             .into(),
//                     )?,
//                 account_id: row
//                     .try_get(
//                         pre,
//                         sea_orm::IdenStatic::as_str(
//                                 &<<Self as sea_orm::ModelTrait>::Entity as sea_orm::entity::EntityTrait>::Column::AccountId,
//                             )
//                             .into(),
//                     )?,
//             })
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::ModelTrait for Model {
//         type Entity = Entity;
//         fn get(
//             &self,
//             c: <Self::Entity as sea_orm::entity::EntityTrait>::Column,
//         ) -> sea_orm::Value {
//             match c {
//                 <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id => {
//                     self.id.clone().into()
//                 }
//                 <Self::Entity as sea_orm::entity::EntityTrait>::Column::AccountId => {
//                     self.account_id.clone().into()
//                 }
//                 _ => {
//                     ::core::panicking::panic_fmt(
//                         ::core::fmt::Arguments::new_v1(
//                             &["field does not exist on Model"],
//                             &[],
//                         ),
//                     )
//                 }
//             }
//         }
//         fn set(
//             &mut self,
//             c: <Self::Entity as sea_orm::entity::EntityTrait>::Column,
//             v: sea_orm::Value,
//         ) {
//             match c {
//                 <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id => {
//                     self.id = v.unwrap();
//                 }
//                 <Self::Entity as sea_orm::entity::EntityTrait>::Column::AccountId => {
//                     self.account_id = v.unwrap();
//                 }
//                 _ => {
//                     ::core::panicking::panic_fmt(
//                         ::core::fmt::Arguments::new_v1(
//                             &["field does not exist on Model"],
//                             &[],
//                         ),
//                     )
//                 }
//             }
//         }
//     }
//     pub struct ActiveModel {
//         pub id: sea_orm::ActiveValue<OpenDepartmentId>,
//         pub account_id: sea_orm::ActiveValue<AccountId>,
//     }
//     #[automatically_derived]
//     impl ::core::clone::Clone for ActiveModel {
//         #[inline]
//         fn clone(&self) -> ActiveModel {
//             ActiveModel {
//                 id: ::core::clone::Clone::clone(&self.id),
//                 account_id: ::core::clone::Clone::clone(&self.account_id),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for ActiveModel {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::debug_struct_field2_finish(
//                 f,
//                 "ActiveModel",
//                 "id",
//                 &&self.id,
//                 "account_id",
//                 &&self.account_id,
//             )
//         }
//     }
//     impl ::core::marker::StructuralPartialEq for ActiveModel {}
//     #[automatically_derived]
//     impl ::core::cmp::PartialEq for ActiveModel {
//         #[inline]
//         fn eq(&self, other: &ActiveModel) -> bool {
//             self.id == other.id && self.account_id == other.account_id
//         }
//     }
//     #[automatically_derived]
//     impl std::default::Default for ActiveModel {
//         fn default() -> Self {
//             <Self as sea_orm::ActiveModelBehavior>::new()
//         }
//     }
//     #[automatically_derived]
//     impl std::convert::From<<Entity as EntityTrait>::Model> for ActiveModel {
//         fn from(m: <Entity as EntityTrait>::Model) -> Self {
//             Self {
//                 id: sea_orm::ActiveValue::unchanged(m.id),
//                 account_id: sea_orm::ActiveValue::unchanged(m.account_id),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::IntoActiveModel<ActiveModel> for <Entity as EntityTrait>::Model {
//         fn into_active_model(self) -> ActiveModel {
//             self.into()
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::ActiveModelTrait for ActiveModel {
//         type Entity = Entity;
//         fn take(
//             &mut self,
//             c: <Self::Entity as EntityTrait>::Column,
//         ) -> sea_orm::ActiveValue<sea_orm::Value> {
//             match c {
//                 <Self::Entity as EntityTrait>::Column::Id => {
//                     let mut value = sea_orm::ActiveValue::not_set();
//                     std::mem::swap(&mut value, &mut self.id);
//                     value.into_wrapped_value()
//                 }
//                 <Self::Entity as EntityTrait>::Column::AccountId => {
//                     let mut value = sea_orm::ActiveValue::not_set();
//                     std::mem::swap(&mut value, &mut self.account_id);
//                     value.into_wrapped_value()
//                 }
//                 _ => sea_orm::ActiveValue::not_set(),
//             }
//         }
//         fn get(
//             &self,
//             c: <Self::Entity as EntityTrait>::Column,
//         ) -> sea_orm::ActiveValue<sea_orm::Value> {
//             match c {
//                 <Self::Entity as EntityTrait>::Column::Id => {
//                     self.id.clone().into_wrapped_value()
//                 }
//                 <Self::Entity as EntityTrait>::Column::AccountId => {
//                     self.account_id.clone().into_wrapped_value()
//                 }
//                 _ => sea_orm::ActiveValue::not_set(),
//             }
//         }
//         fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: sea_orm::Value) {
//             match c {
//                 <Self::Entity as EntityTrait>::Column::Id => {
//                     self.id = sea_orm::ActiveValue::set(v.unwrap());
//                 }
//                 <Self::Entity as EntityTrait>::Column::AccountId => {
//                     self.account_id = sea_orm::ActiveValue::set(v.unwrap());
//                 }
//                 _ => {
//                     ::core::panicking::panic_fmt(
//                         ::core::fmt::Arguments::new_v1(
//                             &["This ActiveModel does not have this field"],
//                             &[],
//                         ),
//                     )
//                 }
//             }
//         }
//         fn not_set(&mut self, c: <Self::Entity as EntityTrait>::Column) {
//             match c {
//                 <Self::Entity as EntityTrait>::Column::Id => {
//                     self.id = sea_orm::ActiveValue::not_set();
//                 }
//                 <Self::Entity as EntityTrait>::Column::AccountId => {
//                     self.account_id = sea_orm::ActiveValue::not_set();
//                 }
//                 _ => {}
//             }
//         }
//         fn is_not_set(&self, c: <Self::Entity as EntityTrait>::Column) -> bool {
//             match c {
//                 <Self::Entity as EntityTrait>::Column::Id => self.id.is_not_set(),
//                 <Self::Entity as EntityTrait>::Column::AccountId => {
//                     self.account_id.is_not_set()
//                 }
//                 _ => {
//                     ::core::panicking::panic_fmt(
//                         ::core::fmt::Arguments::new_v1(
//                             &["This ActiveModel does not have this field"],
//                             &[],
//                         ),
//                     )
//                 }
//             }
//         }
//         fn default() -> Self {
//             Self {
//                 id: sea_orm::ActiveValue::not_set(),
//                 account_id: sea_orm::ActiveValue::not_set(),
//             }
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl Model {
//         #[inline]
//         #[allow(missing_docs)]
//         pub async fn id(
//             &self,
//             ctx: &async_graphql::Context<'_>,
//         ) -> async_graphql::Result<&OpenDepartmentId> {
//             ::std::result::Result::Ok(&self.id)
//         }
//         #[inline]
//         #[allow(missing_docs)]
//         pub async fn account_id(
//             &self,
//             ctx: &async_graphql::Context<'_>,
//         ) -> async_graphql::Result<&AccountId> {
//             ::std::result::Result::Ok(&self.account_id)
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl async_graphql::resolver_utils::ContainerType for Model {
//         #[allow(
//             clippy::let_unit_value,
//             clippy::no_effect_underscore_binding,
//             clippy::shadow_same,
//             clippy::type_complexity,
//             clippy::type_repetition_in_bounds,
//             clippy::used_underscore_binding
//         )]
//         fn resolve_field<'life0, 'life1, 'life2, 'async_trait>(
//             &'life0 self,
//             ctx: &'life1 async_graphql::Context<'life2>,
//         ) -> ::core::pin::Pin<
//             Box<
//                 dyn ::core::future::Future<
//                     Output = async_graphql::ServerResult<
//                         ::std::option::Option<async_graphql::Value>,
//                     >,
//                 > + ::core::marker::Send + 'async_trait,
//             >,
//         >
//         where
//             'life0: 'async_trait,
//             'life1: 'async_trait,
//             'life2: 'async_trait,
//             Self: 'async_trait,
//         {
//             Box::pin(async move {
//                 if let ::core::option::Option::Some(__ret)
//                     = ::core::option::Option::None::<
//                         async_graphql::ServerResult<
//                             ::std::option::Option<async_graphql::Value>,
//                         >,
//                     > {
//                     return __ret;
//                 }
//                 let __self = self;
//                 let ctx = ctx;
//                 let __ret: async_graphql::ServerResult<
//                     ::std::option::Option<async_graphql::Value>,
//                 > = {
//                     if ctx.item.node.name.node == "id" {
//                         let f = async move {
//                             __self
//                                 .id(ctx)
//                                 .await
//                                 .map_err(|err| err.into_server_error(ctx.item.pos))
//                         };
//                         let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
//                         let ctx_obj = ctx
//                             .with_selection_set(&ctx.item.node.selection_set);
//                         return async_graphql::OutputType::resolve(
//                                 &obj,
//                                 &ctx_obj,
//                                 ctx.item,
//                             )
//                             .await
//                             .map(::std::option::Option::Some);
//                     }
//                     if ctx.item.node.name.node == "accountId" {
//                         let f = async move {
//                             __self
//                                 .account_id(ctx)
//                                 .await
//                                 .map_err(|err| err.into_server_error(ctx.item.pos))
//                         };
//                         let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
//                         let ctx_obj = ctx
//                             .with_selection_set(&ctx.item.node.selection_set);
//                         return async_graphql::OutputType::resolve(
//                                 &obj,
//                                 &ctx_obj,
//                                 ctx.item,
//                             )
//                             .await
//                             .map(::std::option::Option::Some);
//                     }
//                     if let Some(value)
//                         = <Self as async_graphql::ComplexObject>::resolve_field(
//                                 __self,
//                                 ctx,
//                             )
//                             .await?
//                     {
//                         return Ok(Some(value));
//                     }
//                     ::std::result::Result::Ok(::std::option::Option::None)
//                 };
//                 #[allow(unreachable_code)] __ret
//             })
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl async_graphql::OutputType for Model {
//         fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
//             ::std::borrow::Cow::Borrowed("OrgAccount")
//         }
//         fn create_type_info(
//             registry: &mut async_graphql::registry::Registry,
//         ) -> ::std::string::String {
//             registry
//                 .create_output_type::<
//                     Self,
//                     _,
//                 >(
//                     async_graphql::registry::MetaTypeId::Object,
//                     |registry| async_graphql::registry::MetaType::Object {
//                         name: ::std::borrow::Cow::into_owned(
//                             ::std::borrow::Cow::Borrowed("OrgAccount"),
//                         ),
//                         description: ::std::option::Option::None,
//                         fields: {
//                             let mut fields = async_graphql::indexmap::IndexMap::new();
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("id"),
//                                     async_graphql::registry::MetaField {
//                                         name: ::std::borrow::ToOwned::to_owned("id"),
//                                         description: ::std::option::Option::None,
//                                         args: ::std::default::Default::default(),
//                                         ty: <OpenDepartmentId as async_graphql::OutputType>::create_type_info(
//                                             registry,
//                                         ),
//                                         deprecation: async_graphql::registry::Deprecation::NoDeprecated,
//                                         cache_control: async_graphql::CacheControl {
//                                             public: true,
//                                             max_age: 0i32,
//                                         },
//                                         external: false,
//                                         provides: ::std::option::Option::None,
//                                         requires: ::std::option::Option::None,
//                                         shareable: false,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         override_from: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         compute_complexity: ::std::option::Option::None,
//                                     },
//                                 );
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("accountId"),
//                                     async_graphql::registry::MetaField {
//                                         name: ::std::borrow::ToOwned::to_owned("accountId"),
//                                         description: ::std::option::Option::None,
//                                         args: ::std::default::Default::default(),
//                                         ty: <AccountId as async_graphql::OutputType>::create_type_info(
//                                             registry,
//                                         ),
//                                         deprecation: async_graphql::registry::Deprecation::NoDeprecated,
//                                         cache_control: async_graphql::CacheControl {
//                                             public: true,
//                                             max_age: 0i32,
//                                         },
//                                         external: false,
//                                         provides: ::std::option::Option::None,
//                                         requires: ::std::option::Option::None,
//                                         shareable: false,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         override_from: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         compute_complexity: ::std::option::Option::None,
//                                     },
//                                 );
//                             fields
//                                 .extend(
//                                     <Self as async_graphql::ComplexObject>::fields(registry),
//                                 );
//                             fields
//                         },
//                         cache_control: async_graphql::CacheControl {
//                             public: true,
//                             max_age: 0i32,
//                         },
//                         extends: false,
//                         shareable: false,
//                         inaccessible: false,
//                         tags: &[],
//                         keys: ::std::option::Option::None,
//                         visible: ::std::option::Option::None,
//                         is_subscription: false,
//                         rust_typename: ::std::any::type_name::<Self>(),
//                     },
//                 )
//         }
//         #[allow(
//             clippy::let_unit_value,
//             clippy::no_effect_underscore_binding,
//             clippy::shadow_same,
//             clippy::type_complexity,
//             clippy::type_repetition_in_bounds,
//             clippy::used_underscore_binding
//         )]
//         fn resolve<'life0, 'life1, 'life2, 'life3, 'async_trait>(
//             &'life0 self,
//             ctx: &'life1 async_graphql::ContextSelectionSet<'life2>,
//             _field: &'life3 async_graphql::Positioned<
//                 async_graphql::parser::types::Field,
//             >,
//         ) -> ::core::pin::Pin<
//             Box<
//                 dyn ::core::future::Future<
//                     Output = async_graphql::ServerResult<async_graphql::Value>,
//                 > + ::core::marker::Send + 'async_trait,
//             >,
//         >
//         where
//             'life0: 'async_trait,
//             'life1: 'async_trait,
//             'life2: 'async_trait,
//             'life3: 'async_trait,
//             Self: 'async_trait,
//         {
//             Box::pin(async move {
//                 if let ::core::option::Option::Some(__ret)
//                     = ::core::option::Option::None::<
//                         async_graphql::ServerResult<async_graphql::Value>,
//                     > {
//                     return __ret;
//                 }
//                 let __self = self;
//                 let ctx = ctx;
//                 let _field = _field;
//                 let __ret: async_graphql::ServerResult<async_graphql::Value> = {
//                     async_graphql::resolver_utils::resolve_container(ctx, __self).await
//                 };
//                 #[allow(unreachable_code)] __ret
//             })
//         }
//     }
//     impl async_graphql::ObjectType for Model {}
//     #[graphql(name = "OrgAccountFilter")]
//     pub struct Filter {
//         pub or: Option<Vec<Box<Filter>>>,
//         pub and: Option<Vec<Box<Filter>>>,
//         id: Option<seaography::TypeFilter<OpenDepartmentId>>,
//         account_id: Option<seaography::TypeFilter<AccountId>>,
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for Filter {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::debug_struct_field4_finish(
//                 f,
//                 "Filter",
//                 "or",
//                 &&self.or,
//                 "and",
//                 &&self.and,
//                 "id",
//                 &&self.id,
//                 "account_id",
//                 &&self.account_id,
//             )
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl async_graphql::InputType for Filter {
//         type RawValueType = Self;
//         fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
//             ::std::borrow::Cow::Borrowed("OrgAccountFilter")
//         }
//         fn create_type_info(
//             registry: &mut async_graphql::registry::Registry,
//         ) -> ::std::string::String {
//             registry
//                 .create_input_type::<
//                     Self,
//                     _,
//                 >(
//                     async_graphql::registry::MetaTypeId::InputObject,
//                     |registry| async_graphql::registry::MetaType::InputObject {
//                         name: ::std::borrow::Cow::into_owned(
//                             ::std::borrow::Cow::Borrowed("OrgAccountFilter"),
//                         ),
//                         description: ::std::option::Option::None,
//                         input_fields: {
//                             let mut fields = async_graphql::indexmap::IndexMap::new();
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("or"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "or",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             Vec<Box<Filter>>,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("and"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "and",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             Vec<Box<Filter>>,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("id"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "id",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             seaography::TypeFilter<OpenDepartmentId>,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("accountId"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "accountId",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             seaography::TypeFilter<AccountId>,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                         },
//                         visible: ::std::option::Option::None,
//                         inaccessible: false,
//                         tags: &[],
//                         rust_typename: ::std::any::type_name::<Self>(),
//                         oneof: false,
//                     },
//                 )
//         }
//         fn parse(
//             value: ::std::option::Option<async_graphql::Value>,
//         ) -> async_graphql::InputValueResult<Self> {
//             if let ::std::option::Option::Some(async_graphql::Value::Object(obj))
//                 = value {
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut or: Option<Vec<Box<Filter>>> = async_graphql::InputType::parse(
//                         obj.get("or").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut and: Option<Vec<Box<Filter>>> = async_graphql::InputType::parse(
//                         obj.get("and").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut id: Option<seaography::TypeFilter<OpenDepartmentId>> = async_graphql::InputType::parse(
//                         obj.get("id").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut account_id: Option<seaography::TypeFilter<AccountId>> = async_graphql::InputType::parse(
//                         obj.get("accountId").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 let obj = Self { or, and, id, account_id };
//                 ::std::result::Result::Ok(obj)
//             } else {
//                 ::std::result::Result::Err(
//                     async_graphql::InputValueError::expected_type(
//                         value.unwrap_or_default(),
//                     ),
//                 )
//             }
//         }
//         fn to_value(&self) -> async_graphql::Value {
//             let mut map = async_graphql::indexmap::IndexMap::new();
//             map.insert(
//                 async_graphql::Name::new("or"),
//                 async_graphql::InputType::to_value(&self.or),
//             );
//             map.insert(
//                 async_graphql::Name::new("and"),
//                 async_graphql::InputType::to_value(&self.and),
//             );
//             map.insert(
//                 async_graphql::Name::new("id"),
//                 async_graphql::InputType::to_value(&self.id),
//             );
//             map.insert(
//                 async_graphql::Name::new("accountId"),
//                 async_graphql::InputType::to_value(&self.account_id),
//             );
//             async_graphql::Value::Object(map)
//         }
//         fn federation_fields() -> ::std::option::Option<::std::string::String> {
//             let mut res = ::std::vec::Vec::new();
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     Vec<Box<Filter>>,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"or"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("or"));
//             }
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     Vec<Box<Filter>>,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"and"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("and"));
//             }
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     seaography::TypeFilter<OpenDepartmentId>,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"id"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("id"));
//             }
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     seaography::TypeFilter<AccountId>,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"accountId"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("accountId"));
//             }
//             ::std::option::Option::Some({
//                 let res = ::alloc::fmt::format(
//                     ::core::fmt::Arguments::new_v1(
//                         &["{ ", " }"],
//                         &[::core::fmt::ArgumentV1::new_display(&res.join(" "))],
//                     ),
//                 );
//                 res
//             })
//         }
//         fn as_raw_value(&self) -> ::std::option::Option<&Self::RawValueType> {
//             ::std::option::Option::Some(self)
//         }
//     }
//     impl async_graphql::InputObjectType for Filter {}
//     pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
//         let mut condition = sea_orm::Condition::all();
//         if let Some(current_filter) = root_filter {
//             if let Some(or_filters) = current_filter.or {
//                 let or_condition = or_filters
//                     .into_iter()
//                     .fold(
//                         sea_orm::Condition::any(),
//                         |fold_condition, filter| {
//                             fold_condition.add(filter_recursive(Some(*filter)))
//                         },
//                     );
//                 condition = condition.add(or_condition);
//             }
//             if let Some(and_filters) = current_filter.and {
//                 let and_condition = and_filters
//                     .into_iter()
//                     .fold(
//                         sea_orm::Condition::all(),
//                         |fold_condition, filter| {
//                             fold_condition.add(filter_recursive(Some(*filter)))
//                         },
//                     );
//                 condition = condition.add(and_condition);
//             }
//             if let Some(id) = current_filter.id {
//                 if let Some(eq_value) = id.eq {
//                     condition = condition.add(Column::Id.eq(eq_value));
//                 }
//                 if let Some(ne_value) = id.ne {
//                     condition = condition.add(Column::Id.ne(ne_value));
//                 }
//                 if let Some(gt_value) = id.gt {
//                     condition = condition.add(Column::Id.gt(gt_value));
//                 }
//                 if let Some(gte_value) = id.gte {
//                     condition = condition.add(Column::Id.gte(gte_value));
//                 }
//                 if let Some(lt_value) = id.lt {
//                     condition = condition.add(Column::Id.lt(lt_value));
//                 }
//                 if let Some(lte_value) = id.lte {
//                     condition = condition.add(Column::Id.lte(lte_value));
//                 }
//                 if let Some(is_in_value) = id.is_in {
//                     condition = condition.add(Column::Id.is_in(is_in_value));
//                 }
//                 if let Some(is_not_in_value) = id.is_not_in {
//                     condition = condition.add(Column::Id.is_not_in(is_not_in_value));
//                 }
//                 if let Some(is_null_value) = id.is_null {
//                     if is_null_value {
//                         condition = condition.add(Column::Id.is_null());
//                     }
//                 }
//             }
//             if let Some(account_id) = current_filter.account_id {
//                 if let Some(eq_value) = account_id.eq {
//                     condition = condition.add(Column::AccountId.eq(eq_value));
//                 }
//                 if let Some(ne_value) = account_id.ne {
//                     condition = condition.add(Column::AccountId.ne(ne_value));
//                 }
//                 if let Some(gt_value) = account_id.gt {
//                     condition = condition.add(Column::AccountId.gt(gt_value));
//                 }
//                 if let Some(gte_value) = account_id.gte {
//                     condition = condition.add(Column::AccountId.gte(gte_value));
//                 }
//                 if let Some(lt_value) = account_id.lt {
//                     condition = condition.add(Column::AccountId.lt(lt_value));
//                 }
//                 if let Some(lte_value) = account_id.lte {
//                     condition = condition.add(Column::AccountId.lte(lte_value));
//                 }
//                 if let Some(is_in_value) = account_id.is_in {
//                     condition = condition.add(Column::AccountId.is_in(is_in_value));
//                 }
//                 if let Some(is_not_in_value) = account_id.is_not_in {
//                     condition = condition
//                         .add(Column::AccountId.is_not_in(is_not_in_value));
//                 }
//                 if let Some(is_null_value) = account_id.is_null {
//                     if is_null_value {
//                         condition = condition.add(Column::AccountId.is_null());
//                     }
//                 }
//             }
//         }
//         condition
//     }
//     #[graphql(name = "OrgAccountOrderBy")]
//     pub struct OrderBy {
//         id: Option<seaography::OrderByEnum>,
//         account_id: Option<seaography::OrderByEnum>,
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for OrderBy {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::debug_struct_field2_finish(
//                 f,
//                 "OrderBy",
//                 "id",
//                 &&self.id,
//                 "account_id",
//                 &&self.account_id,
//             )
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl async_graphql::InputType for OrderBy {
//         type RawValueType = Self;
//         fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
//             ::std::borrow::Cow::Borrowed("OrgAccountOrderBy")
//         }
//         fn create_type_info(
//             registry: &mut async_graphql::registry::Registry,
//         ) -> ::std::string::String {
//             registry
//                 .create_input_type::<
//                     Self,
//                     _,
//                 >(
//                     async_graphql::registry::MetaTypeId::InputObject,
//                     |registry| async_graphql::registry::MetaType::InputObject {
//                         name: ::std::borrow::Cow::into_owned(
//                             ::std::borrow::Cow::Borrowed("OrgAccountOrderBy"),
//                         ),
//                         description: ::std::option::Option::None,
//                         input_fields: {
//                             let mut fields = async_graphql::indexmap::IndexMap::new();
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("id"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "id",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             seaography::OrderByEnum,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                                 .insert(
//                                     ::std::borrow::ToOwned::to_owned("accountId"),
//                                     async_graphql::registry::MetaInputValue {
//                                         name: "accountId",
//                                         description: ::std::option::Option::None,
//                                         ty: <Option<
//                                             seaography::OrderByEnum,
//                                         > as async_graphql::InputType>::create_type_info(registry),
//                                         default_value: ::std::option::Option::None,
//                                         visible: ::std::option::Option::None,
//                                         inaccessible: false,
//                                         tags: &[],
//                                         is_secret: false,
//                                     },
//                                 );
//                             fields
//                         },
//                         visible: ::std::option::Option::None,
//                         inaccessible: false,
//                         tags: &[],
//                         rust_typename: ::std::any::type_name::<Self>(),
//                         oneof: false,
//                     },
//                 )
//         }
//         fn parse(
//             value: ::std::option::Option<async_graphql::Value>,
//         ) -> async_graphql::InputValueResult<Self> {
//             if let ::std::option::Option::Some(async_graphql::Value::Object(obj))
//                 = value {
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut id: Option<seaography::OrderByEnum> = async_graphql::InputType::parse(
//                         obj.get("id").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 #[allow(non_snake_case, unused_mut)]
//                 let mut account_id: Option<seaography::OrderByEnum> = async_graphql::InputType::parse(
//                         obj.get("accountId").cloned(),
//                     )
//                     .map_err(async_graphql::InputValueError::propagate)?;
//                 let obj = Self { id, account_id };
//                 ::std::result::Result::Ok(obj)
//             } else {
//                 ::std::result::Result::Err(
//                     async_graphql::InputValueError::expected_type(
//                         value.unwrap_or_default(),
//                     ),
//                 )
//             }
//         }
//         fn to_value(&self) -> async_graphql::Value {
//             let mut map = async_graphql::indexmap::IndexMap::new();
//             map.insert(
//                 async_graphql::Name::new("id"),
//                 async_graphql::InputType::to_value(&self.id),
//             );
//             map.insert(
//                 async_graphql::Name::new("accountId"),
//                 async_graphql::InputType::to_value(&self.account_id),
//             );
//             async_graphql::Value::Object(map)
//         }
//         fn federation_fields() -> ::std::option::Option<::std::string::String> {
//             let mut res = ::std::vec::Vec::new();
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     seaography::OrderByEnum,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"id"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("id"));
//             }
//             if let ::std::option::Option::Some(fields)
//                 = <Option<
//                     seaography::OrderByEnum,
//                 > as async_graphql::InputType>::federation_fields() {
//                 res.push({
//                     let res = ::alloc::fmt::format(
//                         ::core::fmt::Arguments::new_v1(
//                             &["", " "],
//                             &[
//                                 ::core::fmt::ArgumentV1::new_display(&"accountId"),
//                                 ::core::fmt::ArgumentV1::new_display(&fields),
//                             ],
//                         ),
//                     );
//                     res
//                 });
//             } else {
//                 res.push(::std::string::ToString::to_string("accountId"));
//             }
//             ::std::option::Option::Some({
//                 let res = ::alloc::fmt::format(
//                     ::core::fmt::Arguments::new_v1(
//                         &["{ ", " }"],
//                         &[::core::fmt::ArgumentV1::new_display(&res.join(" "))],
//                     ),
//                 );
//                 res
//             })
//         }
//         fn as_raw_value(&self) -> ::std::option::Option<&Self::RawValueType> {
//             ::std::option::Option::Some(self)
//         }
//     }
//     impl async_graphql::InputObjectType for OrderBy {}
//     pub fn order_by(
//         stmt: sea_orm::Select<Entity>,
//         order_by_struct: Option<OrderBy>,
//     ) -> sea_orm::Select<Entity> {
//         use sea_orm::QueryOrder;
//         if let Some(order_by_struct) = order_by_struct {
//             let stmt = if let Some(order_by) = order_by_struct.id {
//                 match order_by {
//                     seaography::OrderByEnum::Asc => {
//                         stmt.order_by(Column::Id, sea_orm::query::Order::Asc)
//                     }
//                     seaography::OrderByEnum::Desc => {
//                         stmt.order_by(Column::Id, sea_orm::query::Order::Desc)
//                     }
//                 }
//             } else {
//                 stmt
//             };
//             let stmt = if let Some(order_by) = order_by_struct.account_id {
//                 match order_by {
//                     seaography::OrderByEnum::Asc => {
//                         stmt.order_by(Column::AccountId, sea_orm::query::Order::Asc)
//                     }
//                     seaography::OrderByEnum::Desc => {
//                         stmt.order_by(Column::AccountId, sea_orm::query::Order::Desc)
//                     }
//                 }
//             } else {
//                 stmt
//             };
//             stmt
//         } else {
//             stmt
//         }
//     }
//     pub enum Relation {
//         #[sea_orm(has_many = "crate::org::Entity")]
//         Org,
//     }
//     #[automatically_derived]
//     impl ::core::marker::Copy for Relation {}
//     #[automatically_derived]
//     impl ::core::clone::Clone for Relation {
//         #[inline]
//         fn clone(&self) -> Relation {
//             *self
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for Relation {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::write_str(f, "Org")
//         }
//     }
//     #[allow(missing_docs)]
//     pub struct RelationIter {
//         idx: usize,
//         back_idx: usize,
//         marker: ::core::marker::PhantomData<()>,
//     }
//     impl RelationIter {
//         fn get(&self, idx: usize) -> Option<Relation> {
//             match idx {
//                 0usize => ::core::option::Option::Some(Relation::Org),
//                 _ => ::core::option::Option::None,
//             }
//         }
//     }
//     impl sea_orm::strum::IntoEnumIterator for Relation {
//         type Iterator = RelationIter;
//         fn iter() -> RelationIter {
//             RelationIter {
//                 idx: 0,
//                 back_idx: 0,
//                 marker: ::core::marker::PhantomData,
//             }
//         }
//     }
//     impl Iterator for RelationIter {
//         type Item = Relation;
//         fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//             self.nth(0)
//         }
//         fn size_hint(&self) -> (usize, Option<usize>) {
//             let t = if self.idx + self.back_idx >= 1usize {
//                 0
//             } else {
//                 1usize - self.idx - self.back_idx
//             };
//             (t, Some(t))
//         }
//         fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
//             let idx = self.idx + n + 1;
//             if idx + self.back_idx > 1usize {
//                 self.idx = 1usize;
//                 None
//             } else {
//                 self.idx = idx;
//                 self.get(idx - 1)
//             }
//         }
//     }
//     impl ExactSizeIterator for RelationIter {
//         fn len(&self) -> usize {
//             self.size_hint().0
//         }
//     }
//     impl DoubleEndedIterator for RelationIter {
//         fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
//             let back_idx = self.back_idx + 1;
//             if self.idx + back_idx > 1usize {
//                 self.back_idx = 1usize;
//                 None
//             } else {
//                 self.back_idx = back_idx;
//                 self.get(1usize - self.back_idx)
//             }
//         }
//     }
//     impl Clone for RelationIter {
//         fn clone(&self) -> RelationIter {
//             RelationIter {
//                 idx: self.idx,
//                 back_idx: self.back_idx,
//                 marker: self.marker.clone(),
//             }
//         }
//     }
//     #[automatically_derived]
//     impl sea_orm::entity::RelationTrait for Relation {
//         fn def(&self) -> sea_orm::entity::RelationDef {
//             match self {
//                 Self::Org => Entity::has_many(crate::org::Entity).into(),
//                 _ => {
//                     ::core::panicking::panic_fmt(
//                         ::core::fmt::Arguments::new_v1(
//                             &["No RelationDef for Relation"],
//                             &[],
//                         ),
//                     )
//                 }
//             }
//         }
//     }
//     pub struct OrgFK(pub sea_orm::Value);
//     #[automatically_derived]
//     impl ::core::clone::Clone for OrgFK {
//         #[inline]
//         fn clone(&self) -> OrgFK {
//             OrgFK(::core::clone::Clone::clone(&self.0))
//         }
//     }
//     #[automatically_derived]
//     impl ::core::fmt::Debug for OrgFK {
//         fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//             ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OrgFK", &&self.0)
//         }
//     }
//     impl PartialEq for OrgFK {
//         fn eq(&self, other: &Self) -> bool {
//             fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
//                 s.match_indices(p).nth(n).map(|(index, _)| s.split_at(index))
//             }
//             let a = {
//                 let res = ::alloc::fmt::format(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_debug(&self.0)],
//                     ),
//                 );
//                 res
//             };
//             let b = {
//                 let res = ::alloc::fmt::format(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_debug(&other.0)],
//                     ),
//                 );
//                 res
//             };
//             let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);
//             let b = split_at_nth_char(b.as_str(), '(', 1).map(|v| v.1);
//             a.eq(&b)
//         }
//     }
//     impl Eq for OrgFK {}
//     impl std::hash::Hash for OrgFK {
//         fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//             fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
//                 s.match_indices(p).nth(n).map(|(index, _)| s.split_at(index))
//             }
//             let a = {
//                 let res = ::alloc::fmt::format(
//                     ::core::fmt::Arguments::new_v1(
//                         &[""],
//                         &[::core::fmt::ArgumentV1::new_debug(&self.0)],
//                     ),
//                 );
//                 res
//             };
//             let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);
//             a.hash(state)
//         }
//     }
//     impl async_graphql::dataloader::Loader<OrgFK> for crate::OrmDataloader {
//         type Value = Vec<crate::org::Model>;
//         type Error = std::sync::Arc<sea_orm::error::DbErr>;
//         #[allow(
//             clippy::let_unit_value,
//             clippy::no_effect_underscore_binding,
//             clippy::shadow_same,
//             clippy::type_complexity,
//             clippy::type_repetition_in_bounds,
//             clippy::used_underscore_binding
//         )]
//         fn load<'life0, 'life1, 'async_trait>(
//             &'life0 self,
//             keys: &'life1 [OrgFK],
//         ) -> ::core::pin::Pin<
//             Box<
//                 dyn ::core::future::Future<
//                     Output = Result<
//                         std::collections::HashMap<OrgFK, Self::Value>,
//                         Self::Error,
//                     >,
//                 > + ::core::marker::Send + 'async_trait,
//             >,
//         >
//         where
//             'life0: 'async_trait,
//             'life1: 'async_trait,
//             Self: 'async_trait,
//         {
//             Box::pin(async move {
//                 if let ::core::option::Option::Some(__ret)
//                     = ::core::option::Option::None::<
//                         Result<
//                             std::collections::HashMap<OrgFK, Self::Value>,
//                             Self::Error,
//                         >,
//                     > {
//                     return __ret;
//                 }
//                 let __self = self;
//                 let keys = keys;
//                 let __ret: Result<
//                     std::collections::HashMap<OrgFK, Self::Value>,
//                     Self::Error,
//                 > = {
//                     use seaography::heck::ToSnakeCase;
//                     use ::std::str::FromStr;
//                     let key_values: Vec<_> = keys
//                         .into_iter()
//                         .map(|key| key.0.to_owned())
//                         .collect();
//                     let to_column: crate::org::Column = crate::org::Column::from_str(
//                             Relation::Org
//                                 .def()
//                                 .to_col
//                                 .to_string()
//                                 .to_snake_case()
//                                 .as_str(),
//                         )
//                         .unwrap();
//                     use seaography::itertools::Itertools;
//                     let data: std::collections::HashMap<OrgFK, Self::Value> = crate::org::Entity::find()
//                         .filter(to_column.is_in(key_values))
//                         .all(&__self.db)
//                         .await?
//                         .into_iter()
//                         .map(|model| {
//                             let key = OrgFK(model.get(to_column));
//                             (key, model)
//                         })
//                         .into_group_map();
//                     Ok(data)
//                 };
//                 #[allow(unreachable_code)] __ret
//             })
//         }
//     }
//     impl Model {
//         pub async fn Org<'a>(
//             &self,
//             ctx: &async_graphql::Context<'a>,
//         ) -> async_graphql::Result<Option<Vec<crate::org::Model>>> {
//             {
//                 ::std::result::Result::Ok(
//                     async move {
//                         let value: Option<Vec<crate::org::Model>> = {
//                             use seaography::heck::ToSnakeCase;
//                             use ::std::str::FromStr;
//                             let data_loader = ctx
//                                 .data::<
//                                     async_graphql::dataloader::DataLoader<crate::OrmDataloader>,
//                                 >()
//                                 .unwrap();
//                             let from_column: Column = Column::from_str(
//                                     Relation::Org
//                                         .def()
//                                         .from_col
//                                         .to_string()
//                                         .to_snake_case()
//                                         .as_str(),
//                                 )
//                                 .unwrap();
//                             let key = OrgFK(self.get(from_column));
//                             let data: Option<_> = data_loader
//                                 .load_one(key)
//                                 .await
//                                 .unwrap();
//                             data
//                         };
//                         value
//                     }
//                         .await,
//                 )
//             }
//         }
//     }
//     #[allow(clippy::all, clippy::pedantic)]
//     impl async_graphql::ComplexObject for Model {
//         fn fields(
//             registry: &mut async_graphql::registry::Registry,
//         ) -> ::std::vec::Vec<
//             (::std::string::String, async_graphql::registry::MetaField),
//         > {
//             let mut fields = ::std::vec::Vec::new();
//             fields
//                 .push((
//                     "org".to_string(),
//                     async_graphql::registry::MetaField {
//                         name: ::std::borrow::ToOwned::to_owned("org"),
//                         description: ::std::option::Option::None,
//                         args: {
//                             let mut args = async_graphql::indexmap::IndexMap::new();
//                             args
//                         },
//                         ty: <Option<
//                             Vec<crate::org::Model>,
//                         > as async_graphql::OutputType>::create_type_info(registry),
//                         deprecation: async_graphql::registry::Deprecation::NoDeprecated,
//                         cache_control: async_graphql::CacheControl {
//                             public: true,
//                             max_age: 0i32,
//                         },
//                         external: false,
//                         provides: ::std::option::Option::None,
//                         requires: ::std::option::Option::None,
//                         shareable: false,
//                         inaccessible: false,
//                         tags: &[],
//                         override_from: ::std::option::Option::None,
//                         visible: ::std::option::Option::None,
//                         compute_complexity: ::std::option::Option::None,
//                     },
//                 ));
//             fields
//         }
//         #[allow(
//             clippy::let_unit_value,
//             clippy::no_effect_underscore_binding,
//             clippy::shadow_same,
//             clippy::type_complexity,
//             clippy::type_repetition_in_bounds,
//             clippy::used_underscore_binding
//         )]
//         fn resolve_field<'life0, 'life1, 'life2, 'async_trait>(
//             &'life0 self,
//             ctx: &'life1 async_graphql::Context<'life2>,
//         ) -> ::core::pin::Pin<
//             Box<
//                 dyn ::core::future::Future<
//                     Output = async_graphql::ServerResult<
//                         ::std::option::Option<async_graphql::Value>,
//                     >,
//                 > + ::core::marker::Send + 'async_trait,
//             >,
//         >
//         where
//             'life0: 'async_trait,
//             'life1: 'async_trait,
//             'life2: 'async_trait,
//             Self: 'async_trait,
//         {
//             Box::pin(async move {
//                 if let ::core::option::Option::Some(__ret)
//                     = ::core::option::Option::None::<
//                         async_graphql::ServerResult<
//                             ::std::option::Option<async_graphql::Value>,
//                         >,
//                     > {
//                     return __ret;
//                 }
//                 let __self = self;
//                 let ctx = ctx;
//                 let __ret: async_graphql::ServerResult<
//                     ::std::option::Option<async_graphql::Value>,
//                 > = {
//                     if ctx.item.node.name.node == "org" {
//                         let f = async move {
//                             {
//                                 let res = __self.Org(ctx).await;
//                                 res.map_err(|err| {
//                                     ::std::convert::Into::<async_graphql::Error>::into(err)
//                                         .into_server_error(ctx.item.pos)
//                                 })
//                             }
//                         };
//                         let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
//                         let ctx_obj = ctx
//                             .with_selection_set(&ctx.item.node.selection_set);
//                         return async_graphql::OutputType::resolve(
//                                 &obj,
//                                 &ctx_obj,
//                                 ctxQ.item,
//                             )
//                             .await
//                             .map(::std::option::Option::Some);
//                     }
//                     ::std::result::Result::Ok(::std::option::Option::None)
//                 };
//                 #[allow(unreachable_code)] __ret
//             })
//         }
//     }
//     impl Related<crate::org::Entity> for Entity {
//         fn to() -> RelationDef {
//             Relation::Org.def()
//         }
//     }
//     impl ActiveModelBehavior for ActiveModel {}
// }
