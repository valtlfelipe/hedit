// Disclosure: I just want bare minimum telemetry to understand usage patterns
// No personal data is collected or stored
// Telemetry can be disabled by the user
// See src-tauri/src/telemetry.rs for more details

import { invoke } from '@tauri-apps/api/core'

export function useTelemetry() {
  const trackEvent = (eventName: string) => {
    return invoke('send_telemetry_event', { event: eventName })
  }

  return {
    trackEvent,
  }
}
