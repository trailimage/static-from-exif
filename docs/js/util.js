"use strict";var util={setting:{save:function(t,e){window.localStorage&&localStorage.setItem(t,e)},load:function(t){return window.localStorage?localStorage.getItem(t):null},set showMapLegend(t){util.setting.save("map-legend",t?"true":"false")},get showMapLegend(){var t=util.setting.load("map-legend");return!t||"true"==t}},html:{icon:function(t,e){var n=$("<i>").addClass("material-icons "+t).text(t);return void 0!==e&&n.click(e),n}},log:{event:function(t,e,n){ga("send","event",t,e,n)}}};
//# sourceMappingURL=/js/maps/util.js.map
