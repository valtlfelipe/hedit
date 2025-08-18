import { zoomUtils } from '../utils/zoomUtils'

export function useZoom() {
  return {
    handleZoomIn: zoomUtils.handleZoomIn,
    handleZoomOut: zoomUtils.handleZoomOut,
    handleZoomReset: zoomUtils.handleZoomReset,
  }
}
