use log::*;
use resalt_models::*;
use resalt_storage::StorageImpl;

pub fn get_jobs(
    data: Box<dyn StorageImpl>,
    paginate: Paginate,
    sort: Option<String>,
) -> Result<Vec<Job>, ApiError> {
    let jobs = match data.list_jobs(sort, paginate) {
        Ok(jobs) => jobs,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(jobs)
}
