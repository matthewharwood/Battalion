class VoteWidget extends HTMLElement {
  constructor() {
    super();
    this.locked = false;
  }

  connectedCallback() {
    this.form = this.querySelector('#vote-form');
    this.buttons = this.form.querySelectorAll('button');
    this.yayCountEl = this.querySelector('#yay-count');
    this.mayCountEl = this.querySelector('#may-count');
    this.nayCountEl = this.querySelector('#nay-count');
    debugger
    this.form.addEventListener('submit', (e) => {
      e.preventDefault();
      if (this.locked) {
        this.animateButtons();
        return;
      }
      const formData = new FormData(this.form);
      const payload = Object.fromEntries(formData.entries());
      payload['score'] = parseInt(e.submitter.value, 10);
      debugger
      fetch('/vote', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      }).then(res => {
        if (res.status === 201) {
          this.locked = true;
        }
        this.animateButtons();
      });
    });
    // debugger
    const ws = new WebSocket(`ws://${location.host}/rpc`);

    ws.onmessage = (msg) => {
      console.log("WebSocket received message", msg.data);
      const data = JSON.parse(msg.data);
      if (data.applicantId === this.form.elements['applicantId'].value) {
        this.updateRarity(data.rarity);
        this.updateCounts(data.counts);
      }
    };
  }

  animateButtons() {
    debugger
    this.buttons.forEach(btn => {
      btn.classList.add('clicked');
      setTimeout(() => btn.classList.remove('clicked'), 300);
    });
  }

  updateCounts(counts) {
    if (!counts) return;
    if (typeof counts.yay !== 'undefined') {
      this.yayCountEl.textContent = counts.yay;
    }
    if (typeof counts.may !== 'undefined') {
      this.mayCountEl.textContent = counts.may;
    }
    if (typeof counts.nay !== 'undefined') {
      this.nayCountEl.textContent = counts.nay;
    }
  }

  updateRarity(val) {
    this.querySelector('#rarity').textContent = `${val}%`;
  }
}

customElements.define('vote-widget', VoteWidget);
