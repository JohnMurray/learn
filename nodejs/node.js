var express = require('express');

var app = express();
app.use(express.bodyParser());

app.get('/creative', function (req, res) { 
  res.end('ahhhhhh... no ID found!');
});


app.get('/creative/:id', function (req, res) { 
  res.end('requested createive with id: ' + req.params.id);
});


app.post('/creative/:id', function (req, res) {
  var out = '';
  if (req.body) {
    out += "adding creative: " + req.body.creative + "\n";
  }
  out += 'done adding creative with process of ' + req.params.id + '!';

  res.end(out);
});


app.listen(8124);
