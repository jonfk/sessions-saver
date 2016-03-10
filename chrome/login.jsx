'use strict';
var React = require('react');
var ReactDOM = require('react-dom');

var EmailInput = React.createClass({
  propTypes: {
    handleChange: React.PropTypes.func.isRequired
  },
  render: function() {
    return (<input
        type="email"
        id="email"
        onChange={this.props.handleChange}
    />);
  }
});

var PasswordInput = React.createClass({
  propTypes: {
    handleChange: React.PropTypes.func.isRequired
  },
  render: function() {
    return (<input
        type="password"
        id="password"
        onChange={this.props.handleChange}
    />);
  }
});

var LoginForm = React.createClass({
  handleSubmit: function(event) {
    event.preventDefault();
    console.log(this.state);
  },
  handleChangeEmail: function(event){
    this.setState({email: event.target.value});
  },
  handleChangePassword: function(event){
    this.setState({password: event.target.value});
  },
  render: function(){
    return (
      <div>
      <h1>Login</h1>
      <form onSubmit={this.handleSubmit}>
        <div>
          <label htmlFor="email">Email:</label>
          <EmailInput handleChange={this.handleChangeEmail} />
        </div>
        <div>
          <label htmlFor="password">Password:</label>
          <PasswordInput handleChange={this.handleChangePassword} />
        </div>
        <div className="button">
          <button type="submit">login</button>
        </div>
      </form>
      </div>
    );
  }
});

var LoginBox = React.createClass({
  render: function(){
    return (
      <div className="loginBox">
        <LoginForm />
      </div>
    );
  }
});


ReactDOM.render(
  <LoginBox />,
  document.getElementById('login')
);
