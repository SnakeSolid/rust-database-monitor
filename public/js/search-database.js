"use strict";


const NO_DATA_MESSAGE = "No data";


function DatabaseItem(name, collate, role, server, updated) {
  var self = this;

  self.name = ko.observable(name);
  self.collate = ko.observable(collate);
  self.role = ko.observable(role);
  self.server = ko.observable(server);

  self.updated = ko.computed(function() {
    if (updated === 0) {
      return "";
    }

    return moment.unix(updated).fromNow();
  });

  self.isOk = ko.computed(function() {
    var now = new Date().getTime() / 1000.0;
    var delta = now - updated;

    return delta <= 15 * 60;
  });

  self.isWarn = ko.computed(function() {
    var now = new Date().getTime() / 1000.0;
    var delta = now - updated;

    return delta > 15 * 60 && delta <= 30 * 60;
  });

  self.isErr = ko.computed(function() {
    var now = new Date().getTime() / 1000.0;
    var delta = now - updated;

    return delta > 30 * 60;
  });
}


function SearchDatabaseModel() {
  var self = this;
  self.updated = ko.observable("now");
  self.query = ko.observable("");
  self.loading = ko.observable(false);
  self.databases = ko.observableArray([]);
  self.message = ko.observable(NO_DATA_MESSAGE);

  self.timerId = null;

  self.loaded = ko.computed(function () {
    return !self.loading();
  });

  self.messageVisible = ko.computed(function () {
    return self.message().length > 0;
  });

  self.tableVisible = ko.computed(function () {
    return self.databases().length > 0;
  });

  self.submit = function() {
    var data = { "query": self.query() };

    $.ajax("/api/v1/databases", {
      data: JSON.stringify(data),
      contentType: 'application/json',
      type: 'POST'
    }).done(function(data) {
      if (data["ok"] === true) {
        var databases = data["databases"] || [];

        self.databases(databases.map(function (item) {
          return new DatabaseItem(
            item["database_name"] || "",
            item["collation_name"] || "",
            item["role_name"] || "",
            item["server_name"] || "",
            item["last_update"] || 0
          );
        }));

        if (databases.length === 0) {
          self.message(NO_DATA_MESSAGE);
        }
      } else {
        self.databases([]);
        self.message(date["message"]);
      }

      self.loading(false);
    }).fail(function() {
      console.log("fail");

      self.loading(false);
    });

    self.message("");
    self.loading(true);
  }

  self.query.subscribe(function(newValue) {
    if (self.timerId != null) {
      window.clearTimeout(self.timerId);
    }

    self.timerId = window.setTimeout(self.submit, 300);
  });
}


$(function() {
  ko.applyBindings(new SearchDatabaseModel());
})
