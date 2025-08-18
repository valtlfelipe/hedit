export const zoomUtils = {
  getCurrentZoom: () => {
    return parseInt(document.body.style.zoom.replace('%', '')) || 100
  },

  setZoom: (zoom: number) => {
    document.body.style.zoom = `${zoom}%`
  },

  handleZoomIn: () => {
    const currentZoom = zoomUtils.getCurrentZoom()
    zoomUtils.setZoom(currentZoom + 10)
  },

  handleZoomOut: () => {
    const currentZoom = zoomUtils.getCurrentZoom()
    zoomUtils.setZoom(currentZoom - 10)
  },

  handleZoomReset: () => {
    zoomUtils.setZoom(100)
  }
}