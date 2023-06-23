use futures::TryStreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::{bson, doc};
use mongodb::options::FindOptions;

use error::make_error;

use crate::profile::entity::{Basics, Filter, Profile};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait ProfileRepository {
    async fn get_random_profiles(
        &self,
        excepted_id: &str,
        filter: Filter,
        count: u64,
    ) -> Result<Vec<Profile>, Error>;
    async fn get_profiles(
        &self,
        excepted_id: &str,
        filter: Filter,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Profile>, Error>;
    async fn get_profile_by_id(&self, id: &str) -> Result<Option<Profile>, Error>;
    async fn create_profile(
        &self,
        id: &str,
        name: &str,
        basics: Basics,
        bio: Option<String>,
    ) -> Result<String, Error>;
    async fn update_profile(
        &self,
        id: &str,
        name: Option<String>,
        basics: Option<Basics>,
        bio: Option<String>,
    ) -> Result<Profile, Error>;
    async fn delete_profile(&self, id: &str) -> Result<String, Error>;
}

pub struct ProfileRepositoryImpl {
    collection: Collection<Profile>,
}

impl ProfileRepositoryImpl {
    pub fn new(collection: Collection<Profile>) -> Box<dyn ProfileRepository + Send + Sync> {
        Box::new(ProfileRepositoryImpl { collection })
    }
}

#[async_trait::async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn get_random_profiles(&self, excepted_id: &str, filter: Filter, count: u64) -> Result<Vec<Profile>, Error> {
        let filter = doc! {
            "_id": {
                "$ne": excepted_id
            },
            "basics.age": {
                "$gte": filter.min_age,
                "$lte": filter.max_age
            },
            "basics.gender": {
                "$in": filter.preferences
            },
            "basics.location": filter.location
        };
        let mut profiles: Vec<Profile> = vec![];
        let pipeline = vec![
            doc! {
                "$match": filter
            },
            doc! {
                "$sample": {
                    "size": count.to_string()
                }
            },
        ];
        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        while let Some(profile) = cursor.try_next().await? {
            let profile = bson::from_document::<Profile>(profile)?;
            profiles.push(profile)
        }
        Ok(profiles)
    }

    async fn get_profiles(&self, excepted_id: &str, filter: Filter, skip: u64, limit: u64) -> Result<Vec<Profile>, Error> {
        let filter = doc! {
            "_id": {
                "$ne": excepted_id
            },
            "basics.age": {
                "$gte": filter.min_age,
                "$lte": filter.max_age
            },
            "basics.gender": {
                "$in": filter.preferences
            },
            "basics.location": filter.location
        };
        let mut profiles: Vec<Profile> = vec![];
        let options = FindOptions::builder().skip(skip).limit(limit.try_into().ok()).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(profile) = cursor.try_next().await? {
            profiles.push(profile)
        }
        Ok(profiles)
    }

    async fn get_profile_by_id(&self, id: &str) -> Result<Option<Profile>, Error> {
        if let Ok(profile) = self.collection.find_one(doc! { "_id": id }, None).await {
            return Ok(profile);
        }
        Err(make_error!("unable to get profile by id"))
    }

    async fn create_profile(&self, id: &str, name: &str, basics: Basics, bio: Option<String>) -> Result<String, Error> {
        let profile = Profile::new(id, name, basics, bio);
        let result = self.collection.insert_one(profile, None).await?;
        if let Some(id) = result.inserted_id.as_str() {
            return Ok(String::from(id));
        }
        Err(make_error!("unable to create profile"))
    }

    async fn update_profile(&self, id: &str, name: Option<String>, basics: Option<Basics>, bio: Option<String>) -> Result<Profile, Error> {
        let timestamp = bson!(Profile::timestamp_now() as i64);
        let mut document = doc! {
            "name": name,
            "bio": bio,
            "updated_at": timestamp
        };
        if let Some(basics) = basics {
            document.insert("basics", bson::to_bson(&basics)?);
        }
        let result = self.collection.update_one(doc! { "_id": id }, doc! { "$set": document }, None).await?;
        if result.modified_count > 0 {
            if let Some(profile) = self.get_profile_by_id(id).await? {
                return Ok(profile);
            }
        }
        Err(make_error!("unable to update profile"))
    }

    async fn delete_profile(&self, id: &str) -> Result<String, Error> {
        if self.collection.delete_one(doc! { "_id": id }, None).await?.deleted_count > 0 {
            return Ok(String::from(id));
        }
        Err(make_error!("unable to delete profile"))
    }
}