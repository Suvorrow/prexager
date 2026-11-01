//! The Inner Voice is Prexager's curiosity engine. Its outputs are either:
//! "questions" (driving the Talented Child to produce reasoning and proposals)
//! or "judgements" (evaluative assessments of the current active reflection traces,
//! and semantic context).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
