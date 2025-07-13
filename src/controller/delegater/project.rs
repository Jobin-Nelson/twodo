use crate::{
    app::Message,
    cli::{ProjectAddArg, ProjectDeleteArg, ProjectEditArg, ProjectOp},
    objects::Project,
    Result,
};
use sqlx::SqlitePool;

pub(crate) async fn delegate_project_op(db: &SqlitePool, op: ProjectOp) -> Result<Message> {
    match op {
        ProjectOp::List => list_project(db, &mut std::io::stdout()).await,
        ProjectOp::Add(add_arg) => add_project(db, add_arg).await,
        ProjectOp::Edit(project_edit_arg) => edit_project(db, project_edit_arg).await,
        ProjectOp::Delete(project_delete_arg) => delete_project(db, project_delete_arg).await,
    }
}

async fn list_project(db: &SqlitePool, mut writer: impl std::io::Write) -> Result<Message> {
    let projects: Vec<Project> = sqlx::query_as("SELECT * FROM projects")
        .fetch_all(db)
        .await?;

    for project in projects {
        writeln!(writer, "{}. {}", project.id, project.name)?;
    }

    Ok(Message::Noop)
}

async fn add_project(db: &SqlitePool, add_arg: ProjectAddArg) -> Result<Message> {
    sqlx::query(
        "INSERT INTO projects (name)
        VALUES (?1)",
    )
    .bind(add_arg.name)
    .execute(db)
    .await?;

    Ok(Message::Noop)
}

async fn edit_project(db: &SqlitePool, edit_arg: ProjectEditArg) -> Result<Message> {
    sqlx::query::<sqlx::Sqlite>("UPDATE projects SET name = ? WHERE id = ?")
        .bind(edit_arg.name)
        .bind(edit_arg.id)
        .execute(db)
        .await?;
    Ok(Message::Noop)
}

async fn delete_project(db: &SqlitePool, edit_arg: ProjectDeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM projects WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    async fn init_db() -> Result<sqlx::SqlitePool> {
        let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
        // create table
        sqlx::migrate!("./migrations").run(&db).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn test_add_project() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;

        // -- Exec
        let project_name = "Test add project";
        let add_arg = ProjectOp::Add(ProjectAddArg {
            name: project_name.to_string(),
        });
        delegate_project_op(&db, add_arg).await?;

        // -- Check
        let project: Project = sqlx::query_as("SELECT * FROM PROJECTS WHERE name = ?1")
            .bind(project_name)
            .fetch_one(&db)
            .await?;

        let result_name: String = project.name;
        assert_eq!(result_name, project_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_project() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let project_name = "Test edit project";
        let add_arg = ProjectOp::Add(ProjectAddArg {
            name: project_name.to_string(),
        });
        delegate_project_op(&db, add_arg).await?;

        // -- Exec
        let edited_project_id = 2;

        let edited_project_name = "This is the new edited project";
        let edit_arg = ProjectOp::Edit(ProjectEditArg {
            id: edited_project_id,
            name: edited_project_name.to_string(),
        });
        delegate_project_op(&db, edit_arg).await?;

        // -- Check
        let project: Project = sqlx::query_as("SELECT * FROM projects WHERE name = ?1")
            .bind(edited_project_name)
            .fetch_one(&db)
            .await?;

        let result_name: String = project.name;
        assert_eq!(result_name, edited_project_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_project() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let project_name = "Test delete project";
        let add_arg = ProjectOp::Add(ProjectAddArg {
            name: project_name.to_string(),
        });
        delegate_project_op(&db, add_arg).await?;

        // -- Exec
        let project_id = 2;
        let delete_arg = ProjectOp::Delete(ProjectDeleteArg { id: project_id });
        delegate_project_op(&db, delete_arg).await?;

        // -- Check
        let task: Option<Project> = sqlx::query_as("SELECT * FROM projects WHERE name = ?1")
            .bind(project_name)
            .fetch_optional(&db)
            .await?;

        assert_eq!(None, task);
        Ok(())
    }
}

// endregion: --- Tests
