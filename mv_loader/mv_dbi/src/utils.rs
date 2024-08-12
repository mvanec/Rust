use uuid;
use uuid::Uuid;

///
/// Make a UUID based on encoding the provided String referenc
///
/// This will creat a V5 UUID in namespace OID using the bytes from
/// the String value.
///
/// OID namespace = {6ba7b812-9dad-11d1-80b4-00c04fd430c8}
pub fn make_uuid(value: &String) -> Uuid {
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, value.as_bytes());
    uuid
}
