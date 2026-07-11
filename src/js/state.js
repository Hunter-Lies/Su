// state.js — shared DOM refs and variables for Su!
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// DOM refs — populated by initDomRefs()
var dropZone, fileList, emptyHint, clearAllBtn;
var qrModal, qrModalImg, qrModalName, qrModalUrl, qrModalCopy;
var sendQrWrap, toastEl, toastTimer;
var qrCache = {};
var downloadsDir, pickDirBtn, themeDd, themeDdLabel, styleDd, styleDdLabel, autostartToggle, shortcutBtn, settingsIp;

// Settings toggle refs
var soundToggle, soundDd, soundDdBtn, soundDdLabel, soundDdMenu;
var popupToggle, trayToggle, clearReceivedToggle;

function initDomRefs() {
  dropZone = document.getElementById("drop-zone");
  fileList = document.getElementById("file-list");
  emptyHint = document.getElementById("empty-hint");
  clearAllBtn = document.getElementById("clear-all-btn");
  qrModal = document.getElementById("qr-modal");
  qrModalImg = document.getElementById("qr-modal-img");
  qrModalName = document.getElementById("qr-modal-name");
  qrModalUrl = document.getElementById("qr-modal-url");
  qrModalCopy = document.getElementById("qr-modal-copy");
  sendQrWrap = document.getElementById("send-qr-wrap");
  toastEl = document.getElementById("toast");
  downloadsDir = document.getElementById("downloads-dir");
  pickDirBtn = document.getElementById("pick-dir-btn");
  themeDd = document.getElementById("theme-dd");
  themeDdLabel = document.getElementById("theme-dd-label");
  styleDd = document.getElementById("style-dd");
  styleDdLabel = document.getElementById("style-dd-label");
  autostartToggle = document.getElementById("autostart-toggle");
  shortcutBtn = document.getElementById("shortcut-btn");
  settingsIp = document.getElementById("settings-ip");
  // Settings toggles
  soundToggle = document.getElementById("sound-toggle");
  soundDd = document.getElementById("sound-dd");
  soundDdBtn = document.getElementById("sound-dd-btn");
  soundDdLabel = document.getElementById("sound-dd-label");
  soundDdMenu = document.getElementById("sound-dd-menu");
  popupToggle = document.getElementById("popup-toggle");
  trayToggle = document.getElementById("tray-toggle");
  clearReceivedToggle = document.getElementById("clear-received-toggle");
}

export { invoke, listen, dropZone, fileList, emptyHint, clearAllBtn, qrModal, qrModalImg, qrModalName, qrModalUrl, qrModalCopy, sendQrWrap, toastEl, toastTimer, qrCache, downloadsDir, pickDirBtn, themeDd, themeDdLabel, styleDd, styleDdLabel, autostartToggle, shortcutBtn, settingsIp, soundToggle, soundDd, soundDdBtn, soundDdLabel, soundDdMenu, popupToggle, trayToggle, clearReceivedToggle, initDomRefs };