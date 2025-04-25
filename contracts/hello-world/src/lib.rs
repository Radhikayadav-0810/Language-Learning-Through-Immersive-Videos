#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

// Structure to store video learning session data
#[contracttype]
#[derive(Clone)]
pub struct VideoSession {
    pub session_id: u64,
    pub language: String,
    pub video_title: String,
    pub timestamp: u64,
    pub completed: bool,
}

// Constants
const SESSION_COUNT: Symbol = symbol_short!("SESH_CNT"); // Shortened symbol

// Storage key
#[contracttype]
pub enum SessionKey {
    Session(u64),
}

#[contract]
pub struct LanguageLearningContract;

#[contractimpl]
impl LanguageLearningContract {
    // Function to create a new video session
    pub fn create_session(env: Env, language: String, video_title: String) -> u64 {
        let mut count = env.storage().instance().get(&SESSION_COUNT).unwrap_or(0u64);
        count += 1;

        let session = VideoSession {
            session_id: count,
            language,
            video_title,
            timestamp: env.ledger().timestamp(),
            completed: false,
        };

        env.storage().instance().set(&SessionKey::Session(count), &session);
        env.storage().instance().set(&SESSION_COUNT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        count
    }

    // Function to mark a session as completed
    pub fn complete_session(env: Env, session_id: u64) {
        let key = SessionKey::Session(session_id);
        let mut session: VideoSession = env.storage().instance().get(&key).expect("Session not found");
        session.completed = true;
        env.storage().instance().set(&key, &session);
    }

    // Function to view session data
    pub fn view_session(env: Env, session_id: u64) -> VideoSession {
        env.storage().instance().get(&SessionKey::Session(session_id)).unwrap_or(VideoSession {
            session_id: 0,
            language: String::from_str(&env, "Not_Found"),
            video_title: String::from_str(&env, "Not_Found"),
            timestamp: 0,
            completed: false,
        })
    }
}