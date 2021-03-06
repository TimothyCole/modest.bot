use crate::discord::shard::ShardManagerContainer;
use serenity::{
  client::bridge::gateway::ShardId,
  framework::standard::{macros::command, CommandResult},
  model::prelude::Message,
  prelude::Context,
};

#[command]
#[owners_only]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  let data = ctx.data.read().await;
  let shard_manager = match data.get::<ShardManagerContainer>() {
    Some(v) => v,
    None => {
      log::error!("Error getting the shard manager.");
      return Ok(());
    }
  };

  let manager = shard_manager.lock().await;
  let runners = manager.runners.lock().await;
  let runner = match runners.get(&ShardId(ctx.shard_id)) {
    Some(runner) => runner,
    None => {
      log::error!("No shards found.");
      return Ok(());
    }
  };

  match runner.latency {
    Some(x) => {
      msg
        .channel_id
        .say(
          &ctx.http,
          &format!("Shard {} - {}ms", ctx.shard_id + 1, x.as_millis()),
        )
        .await?;
    }
    None => {
      msg
        .channel_id
        .say(&ctx.http, "Too soon to measure.")
        .await?;
    }
  };

  Ok(())
}
