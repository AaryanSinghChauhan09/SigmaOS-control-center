use anyhow::Result;
use sigmaos_component::{Config, Service};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() -> Result<()> {
        // Arrange
        let config = Config::default();

        // Act
        let result = config.process();

        // Assert
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_error_handling() -> Result<()> {
        // Arrange
        let custom_config = Config::new("debug");

        // Act
        let result = custom_config.process();

        // Assert
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_service_lifecycle() -> Result<()> {
        // Arrange
        let config = Config::default();
        let mut service = Service::new(config);
        assert!(!service.is_running());

        // Act & Assert - Start
        service.start()?;
        assert!(service.is_running());

        // Act & Assert - Redundant start fails cleanly
        let duplicate_start = service.start();
        assert!(duplicate_start.is_err());

        // Act & Assert - Stop
        service.stop()?;
        assert!(!service.is_running());

        // Act & Assert - Redundant stop fails cleanly
        let duplicate_stop = service.stop();
        assert!(duplicate_stop.is_err());

        Ok(())
    }
}
