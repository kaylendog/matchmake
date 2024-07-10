//! Defines models for matchmaking data.

use std::future::Future;

use serde::{Deserialize, Serialize};

/// A player in the matchmaking system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// The unique ID of this player.
    pub id: u128,
    /// The rank of this player.
    pub rank: u32,
}

/// A lobby in the matchmaking system.
#[derive(Serialize, Deserialize)]
pub struct Lobby {
    /// The players in this lobby.
    pub players: Vec<Player>,
}

impl Lobby {
    /// Create a new lobby with the given players.
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players: players.into(),
        }
    }

    /// Get the mean rank of the players in this lobby.
    pub fn mean_rank(&self) -> Option<f64> {
        match self.players.iter().map(|p| p.rank as f64).sum::<f64>() {
            0.0 => None,
            sum => Some(sum / self.players.len() as f64),
        }
    }

    /// Get the median rank of the players in this lobby.
    pub fn median_rank(&mut self) -> Option<f64> {
        self.players.sort_by_key(|player| player.rank);
        self.players
            .get(self.players.len() / 2)
            .map(|player| player.rank as f64)
    }
}

/// A trait for providing ranks for players.
pub trait RankProvider {
    /// Get the rank of this player.
    fn get_rank(&self, player: Player) -> impl Future<Output = u32> + Send;
}

#[cfg(test)]
mod tests {
    use crate::{Lobby, Player};

    #[test]
    fn median() {
        let players = vec![
            Player { id: 0, rank: 10 },
            Player { id: 1, rank: 15 },
            Player { id: 2, rank: 5 },
        ];
        let mut lobby = Lobby::new(players);
        assert_eq!(lobby.median_rank(), Some(10.0));
    }

    #[test]
    fn mean() {
        let players = vec![
            Player { id: 0, rank: 12 },
            Player { id: 1, rank: 15 },
            Player { id: 2, rank: 5 },
        ];
        let lobby = Lobby::new(players);
        assert!(lobby.mean_rank().unwrap() - 10.67 < 0.01)
    }
}
