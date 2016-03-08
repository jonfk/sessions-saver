"use strict";

const SESSION_ALARM = 'session-saver';

var pollingPeriodInMinutes = 1;

var superagent = require('superagent');

/**
 * @param {HistoryItem} historyItem -
 *
 * https://developer.chrome.com/extensions/history#event-onVisited
 */
function onHistoryVisited(item) {
  console.log(item);
  chrome.identity.getProfileUserInfo(function (userinfo) {
    console.log(userinfo);
  });

  // TODO send to server
  /* superagent.get('https://www.google.ca')
     .end(function(err, res) {
     console.log(res);
     }); */

}


/* chrome.history.onVisited.addListener(onHistoryVisited); */


chrome.alarms.create(SESSION_ALARM, {periodInMinutes: pollingPeriodInMinutes});

/*
 * Periodically gather info on system and tabs
 */
function alarm (alarm) {
  if (alarm.name == SESSION_ALARM) {
    getSession(saveSession);
  }
}

chrome.alarms.onAlarm.addListener(alarm);

function getSession(callback) {
    chrome.system.memory.getInfo(function (memInfo) {
      chrome.system.cpu.getInfo(function (cpuInfo) {
        let sysInfo = {
          memory: memInfo,
          cpu: cpuInfo
        };
        chrome.tabs.query({}, function (tabs) {

          if ('geolocation' in navigator) {
            navigator.geolocation.getCurrentPosition(function(position) {
              callback(tabs, sysInfo, position);
            });
          } else {
            callback(tabs, sysInfo);
          }

        })
      });
    });
}

function saveSession(tabs, sysInfo, geoposition) {

  let sessionTabs = [];
  for (let i = 0; i < tabs.length; i++) {
    let tab = {
      windowIndex: tabs[i].index,
      highlighted: tabs[i].highlighted,
      pinned: tabs[i].pinned,
      audible: tabs[i].audible,
      url: tabs[i].url,
      title: tabs[i].title,
      incognito: tabs[i].incognito,
      favIconUrl: tabs[i].favIconUrl
    };
    sessionTabs.push(tab);
  }

  let sessionInfo = {tabs: sessionTabs, system: sysInfo};

  if (typeof geoposition !== 'undefined') {
    console.log(geoposition);
    sessionInfo.geoLocation = geoposition.coords;
  }

  console.log(sessionInfo);

  //TODO send to server
}
