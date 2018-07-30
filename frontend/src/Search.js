
import React, { Component } from 'react';
import { Button, Input, Segment, Form } from 'semantic-ui-react';

class SearchInput extends Component {
  render() {
    return (
      <Form.Field>
        <Input placeholder='Gif search'>{this.props.value}</Input>
      </Form.Field>
    )
  }
}

class CaptionSelection extends Component {
  // todo: needs to set one to active by default
  // todo: needs onChange callback
  render() {
    return (
      <Form.Field>
        <label>Has Caption</label>
        <Button.Group>
          <Button size='small'>Any</Button>
          <Button size='small'>Yes</Button>
          <Button size='small'>No</Button>
        </Button.Group>
      </Form.Field>
    )
  }
}

class TypeSelection extends Component {
  // todo: needs to set one to active by default
  // todo: needs onChange callback
  render() {
    return (
      <Form.Field>
        <label>Resource Type</label>
        <Button.Group>
          <Button size='small'>Any</Button>
          <Button size='small'>Gif</Button>
          <Button size='small'>Webm</Button>
        </Button.Group>
      </Form.Field>
    )
  }
}

export class SearchForm extends Component {
  render() {
    return (
      <Segment style={{textAlign: 'left'}}>
        <Form>
          <SearchInput />
          <CaptionSelection />
          <TypeSelection />
          <div>
            <Button primary> Search </Button>
          </div>
        </Form>
      </Segment>
    )
  }
}
