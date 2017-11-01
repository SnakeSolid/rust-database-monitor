"use strict";

requirejs.config({
  baseUrl: "/public/js/",
  paths: {
    knockout: "https://cdnjs.cloudflare.com/ajax/libs/knockout/3.4.2/knockout-min",
    moment: "https://cdnjs.cloudflare.com/ajax/libs/moment.js/2.19.1/moment.min",
    reqwest: "https://cdnjs.cloudflare.com/ajax/libs/reqwest/2.0.5/reqwest.min",
  },
  shim: {
    reqwest: {
      exports: "reqwest"
    },
  },
  waitSeconds: 15,
});

// Start the application logic.
requirejs([ "knockout", "moment", "reqwest" ], function(ko, moment, reqwest) {
  const NO_DATA_MESSAGE = "No data";

  function DatabaseItem(name, collate, role, server, description, updated) {
    var self = this;

    self.name = ko.observable(name);
    self.collate = ko.observable(collate);
    self.role = ko.observable(role);
    self.server = ko.observable(server);
    self.description = ko.observable(description);

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
    self.updated = ko.observable(0);
    self.query = ko.observable("");
    self.loading = ko.observable(false);
    self.databases = ko.observableArray([]);
    self.message = ko.observable(NO_DATA_MESSAGE);

    self.timerId = null;

    self.lastUpdate = ko.computed(function () {
      var updated = self.updated();

      if (updated === 0) {
        return "fail";
      }

      return moment.unix(updated).fromNow();
    });

    self.updateSuccess = ko.computed(function() {
      return self.updated() !== 0;
    });

    self.updateFailed = ko.computed(function() {
      return self.updated() === 0;
    });

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

      reqwest({
        url: "/api/v1/databases",
        method: "post",
        data: JSON.stringify(data),
        type: "json",
        contentType: "application/json"
      }).then(function (resp) {
        if (resp["ok"] === true) {
          var databases = resp["databases"] || [];

          self.databases(databases.map(function (item) {
            return new DatabaseItem(
              item["database_name"] || "",
              item["collation_name"] || "",
              item["role_name"] || "",
              item["server_name"] || "",
              item["server_description"] || "",
              item["last_update"] || 0
            );
          }));

          if (databases.length === 0) {
            self.message(NO_DATA_MESSAGE);
          }
        } else {
          self.databases([]);
          self.message(resp["message"]);
        }

        self.loading(false);
      }).fail(function(err) {
        self.loading(false);
      });

      self.message("");
      self.loading(true);
    }

    self.checkStatus = function () {
      reqwest({
        url: "/api/v1/status",
        method: "post",
      }).then(function (resp) {
        if (resp["ok"] === true) {
          var last_update = resp["last_update"] || 0;

          self.updated(last_update);
        } else {
          self.updated(0);
        }
      }).fail(function(err) {
        self.updated(0);
      });
    };

    self.query.subscribe(function(newValue) {
      if (self.timerId != null) {
        window.clearTimeout(self.timerId);
      }

      self.timerId = window.setTimeout(self.submit, 300);
    });

    // Update status every 3 minutes
    window.setInterval(self.checkStatus, 30 * 1000);

    // Update status after page loaded
    self.checkStatus();
  }

  ko.applyBindings(new SearchDatabaseModel());
}, function (err) {
  console.log(err.requireType);

  if (err.requireType === "timeout") {
    console.log("modules: " + err.requireModules);
  }

  throw err;
});
