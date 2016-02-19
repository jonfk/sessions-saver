"use strict";

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
chrome.tabs.onUpdated.addListener(onTabUpdated)
