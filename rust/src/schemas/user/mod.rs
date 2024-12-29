pub struct Root<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub struct Namespace<'a> {
    pub namespace: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub struct Database<'a> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

// P: any type that implements Serialize
pub struct Record<'a, P> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub access: &'a str,
    pub params: P,
}
