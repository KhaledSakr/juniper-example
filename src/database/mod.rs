// use mongodb::coll::Collection;
// use mongodb::{Bson, bson, doc};
// use schema::types

// pub fn find_one_or_create(collection: Collection, user: String) => schema::types::UserTestGroups | null {
//   let doc = doc! {
//     user,
//   }
//   let userGroups = collection.find_one(Some(doc), None).ok()
//   match userGroups {
//     Some(userGroups) => {
//       return userGroups
//     },
//     None => {
//       let insert_doc = doc! {
//         user: 
//       }
//       collection.insert_one()
//     }
//   }
// }
