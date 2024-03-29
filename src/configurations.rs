pub struct Configurations {}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, String> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {});
        }

        Ok(Configurations {})
    }
}
