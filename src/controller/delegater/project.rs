use crate::{
    Result,
    app::Message,
    cli::{ProjectAddArg, ProjectDeleteArg, ProjectEditArg, ProjectOp},
    objects::Project,
};
use sqlx::SqlitePool;

pub async fn delegate_project_op(db: &SqlitePool, op: ProjectOp) -> Result<Message> {
    match op {
        ProjectOp::List => list_project(db, &mut std::io::stdout()).await,
        ProjectOp::Add(add_arg) => add_project(db, add_arg).await,
        ProjectOp::Edit(project_edit_arg) => edit_project(db, project_edit_arg).await,
        ProjectOp::Delete(project_delete_arg) => delete_project(db, project_delete_arg).await,
    }
}

pub async fn list_project(db: &SqlitePool, mut writer: impl std::io::Write) -> Result<Message> {
    let projects: Vec<Project> = sqlx::query_as("SELECT * FROM projects")
        .fetch_all(db)
        .await?;

    for project in projects {
        writeln!(writer, "{}. {}", project.id, project.name)?;
    }

    Ok(Message::Noop)
}

pub async fn add_project(db: &SqlitePool, add_arg: ProjectAddArg) -> Result<Message> {
    sqlx::query(
        "INSERT INTO projects (name)
        VALUES (?1)",
    )
    .bind(add_arg.name)
    .execute(db)
    .await?;

    Ok(Message::Noop)
}

pub async fn edit_project(db: &SqlitePool, edit_arg: ProjectEditArg) -> Result<Message> {
    sqlx::query::<sqlx::Sqlite>("UPDATE projects SET name = ? WHERE id = ?")
        .bind(edit_arg.name)
        .bind(edit_arg.id)
        .execute(db)
        .await?;
    Ok(Message::Noop)
}

pub async fn delete_project(db: &SqlitePool, edit_arg: ProjectDeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM projects WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}
