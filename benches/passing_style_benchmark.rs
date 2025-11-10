use criterion::{
    criterion_group, criterion_main, Criterion
};
use std::hint::black_box;

use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime,oid::ObjectId};
use utoipa::ToSchema;

// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct UserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectId>,
    pub name: String,
    pub user_type: String,
    pub email: String,
    pub password: String,
    pub matricula: Option<String>,
    pub created_at: MyDateTime,
    pub updated_at: MyDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
// This attribute tells utoipa how to generate the schema for MyObjectId.
#[schema(
    // It should be represented as a simple String type in OpenAPI.
    value_type = String, 
    // Add a description for better documentation.
    description = "A 24-character hexadecimal MongoDB Object ID",
    // Provide a realistic example.
    example = "507f191e810c19729de860ea"
)]
pub struct MyObjectId(pub ObjectId);

impl Deref for MyObjectId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Implement From so you can easily convert an ObjectId into your MyObjectId.
impl From<ObjectId> for MyObjectId {
    fn from(id: ObjectId) -> Self {
        MyObjectId(id)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[schema(
    // It should be represented as a simple String type in OpenAPI.
    value_type = String,
    // Add a description for better documentation.
    description = "A BSON DateTime",
    // Provide a realistic example.
    example = "2021-01-01T00:00:00Z"
)]
pub struct MyDateTime(pub DateTime);


impl Deref for MyDateTime {
    type Target = DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DateTime> for MyDateTime {
    fn from(dt: DateTime) -> Self {
        MyDateTime(dt)
    }
}


// The struct from your question
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSolicitacaoModel {
    pub id_aluno: MyObjectId,
    pub id_professor: MyObjectId,
    pub description: String,
    pub comment: String,
}

// Function that takes the struct by value
fn process_by_value(model: CreateSolicitacaoModel) {
    // black_box prevents the compiler from optimizing away the function call
    black_box(model);
}

// Function that takes the struct by Box
fn process_by_box(model: Box<CreateSolicitacaoModel>) {
    black_box(model);
}

// The benchmark function
fn passing_benchmark(c: &mut Criterion) {
    // Create a sample model to use in the tests
    let model = CreateSolicitacaoModel {
        id_aluno: MyObjectId{0:ObjectId::new()},
        id_professor: MyObjectId{0:ObjectId::new()},
        description: "This is a test description".to_string(),
        comment: "This is a test comment".to_string(),
    };

    let mut group = c.benchmark_group("Pass by Value vs Box");

    // Benchmark the pass-by-value approach
    group.bench_function("Pass by Value", |b| {
        b.iter(|| {
            // We clone the model to ensure each iteration is independent
            process_by_value(model.clone())
        })
    });

    // Benchmark the pass-by-box approach
    // IMPORTANT: We include Box::new() inside the iterator
    // to measure the full cost of this approach.
    group.bench_function("Pass by Box", |b| {
        b.iter(|| {
            let boxed_model = Box::new(model.clone());
            process_by_box(boxed_model)
        })
    });

    group.finish();
}

criterion_group!(benches, passing_benchmark);
criterion_main!(benches);