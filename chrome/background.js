"use strict";

const SESSION_ALARM = 'session-saver';

var pollingPeriodInMinutes = 15;

/**
 * @param {Tab} tab - Gives the state of the tab that was updated.
 *
 * https://developer.chrome.com/extensions/tabs#event-onCreated
 */
function onTabCreated(tab) {
  console.log(tab);
}

/**
 * @param {integer} tabId
 * @param {object} changeInfo - Lists the changes to the state of the tab that was updated.
 * @param {Tab} tab - Gives the state of the tab that was updated.
 *
 * https://developer.chrome.com/extensions/tabs#event-onUpdated
 */
function onTabUpdated(tabId, changeInfo, tab) {
  console.log(tab);
}

chrome.tabs.onCreated.addListener(onTabCreated);
chrome.tabs.onUpdated.addListener(onTabUpdated);

chrome.alarms.create(SESSION_ALARM, {periodInMinutes: pollingPeriodInMinutes});

/*
 * Periodically gather info on system and tabs
 */
function alarm (alarm) {
  console.log(alarm);
  if (alarm.name == SESSION_ALARM) {
    chrome.system.cpu.getInfo(function (info) {
      console.log(info);
    });
    chrome.system.memory.getInfo(function (info) {
      console.log(info);
    });
    chrome.tabs.query({}, function (tabs) {
      console.log(tabs);
    })
  }
}
chrome.alarms.onAlarm.addListener(alarm);
